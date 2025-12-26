use crate::config::Config;
use crate::modules::ModuleRegistry;
use crate::window::FastMenuWindow;
use crate::Commands;
use gtk4::prelude::*;
use gtk4::{gio, glib, Application};
use std::cell::RefCell;

thread_local! {
    static WINDOW: RefCell<Option<FastMenuWindow>> = const { RefCell::new(None) };
}

pub struct FastMenuApp {
    app: Application,
}

impl FastMenuApp {
    pub fn new(app_id: &str) -> Self {
        let app = Application::builder()
            .application_id(app_id)
            .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
            .build();

        Self { app }
    }

    pub fn run_with_command(self, command: Option<Commands>) -> glib::ExitCode {
        let command = std::rc::Rc::new(RefCell::new(command));

        self.app.connect_startup(|_app| {
            // CSS loading moved to on_activate (display not ready here)
        });

        let cmd_for_activate = command.clone();
        self.app.connect_activate(move |app| {
            Self::on_activate(app, cmd_for_activate.borrow().clone());
        });

        self.app.connect_command_line(|app, _cmdline| {
            app.activate();
            glib::ExitCode::SUCCESS
        });

        // Add actions for D-Bus control
        let show_action = gio::SimpleAction::new("show", None);
        show_action.connect_activate(|_, _| {
            WINDOW.with(|w| {
                if let Some(window) = w.borrow().as_ref() {
                    window.show_window();
                }
            });
        });
        self.app.add_action(&show_action);

        let hide_action = gio::SimpleAction::new("hide", None);
        hide_action.connect_activate(|_, _| {
            WINDOW.with(|w| {
                if let Some(window) = w.borrow().as_ref() {
                    window.hide_window();
                }
            });
        });
        self.app.add_action(&hide_action);

        let toggle_action = gio::SimpleAction::new("toggle", None);
        toggle_action.connect_activate(|_, _| {
            WINDOW.with(|w| {
                if let Some(window) = w.borrow().as_ref() {
                    window.toggle_window();
                }
            });
        });
        self.app.add_action(&toggle_action);

        self.app.run()
    }

    fn on_activate(app: &Application, command: Option<Commands>) {
        WINDOW.with(|window_cell| {
            if window_cell.borrow().is_none() {
                let config = Config::load().unwrap_or_default();
                let registry = ModuleRegistry::new(&config);
                let window = FastMenuWindow::new(app, config, registry);
                *window_cell.borrow_mut() = Some(window);

                // Load CSS after window is created (display now exists)
                crate::ui::style::load_css();
            }

            if let Some(window) = window_cell.borrow().as_ref() {
                match command {
                    Some(Commands::Show) | None => window.show_window(),
                    Some(Commands::Hide) => window.hide_window(),
                    Some(Commands::Toggle) => window.toggle_window(),
                }
            }
        });
    }
}
