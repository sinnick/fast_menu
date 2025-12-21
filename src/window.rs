use crate::config::Config;
use crate::modules::{ModuleRegistry, SearchResult};
use crate::ui::result_row::ResultRow;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, Box as GtkBox, Entry, EventControllerKey, Label, ListBox,
    Orientation, ScrolledWindow, SelectionMode, Window,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct FastMenuWindow {
    window: Window,
    entry: Entry,
    list_box: ListBox,
    registry: Rc<RefCell<ModuleRegistry>>,
    results: Rc<RefCell<Vec<SearchResult>>>,
}

impl FastMenuWindow {
    pub fn new(app: &Application, _config: Config, registry: ModuleRegistry) -> Self {
        let window = Window::builder()
            .application(app)
            .title("Fast Menu")
            .default_width(650)
            .default_height(380)
            .decorated(false)
            .resizable(false)
            .modal(true)
            .build();
        window.add_css_class("fast-menu");

        // Main container
        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.add_css_class("main-box");

        // Search box
        let search_box = GtkBox::new(Orientation::Horizontal, 8);
        search_box.add_css_class("search-box");
        search_box.set_margin_start(16);
        search_box.set_margin_end(16);
        search_box.set_margin_top(10);
        search_box.set_margin_bottom(10);

        let search_icon = Label::new(Some("❯"));
        search_icon.add_css_class("search-icon");
        search_box.append(&search_icon);

        let entry = Entry::builder()
            .placeholder_text("")
            .hexpand(true)
            .has_frame(false)
            .build();
        entry.add_css_class("search-entry");
        search_box.append(&entry);

        // Separator after search
        let sep1 = GtkBox::new(Orientation::Horizontal, 0);
        sep1.add_css_class("separator");

        main_box.append(&search_box);
        main_box.append(&sep1);

        // Results list
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .vexpand(true)
            .build();
        scrolled.add_css_class("results-scroll");

        let list_box = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .build();
        list_box.add_css_class("results-list");

        scrolled.set_child(Some(&list_box));
        main_box.append(&scrolled);

        // Separator before footer
        let sep2 = GtkBox::new(Orientation::Horizontal, 0);
        sep2.add_css_class("separator");
        main_box.append(&sep2);

        // Footer
        let footer = GtkBox::new(Orientation::Horizontal, 24);
        footer.add_css_class("footer");
        footer.set_margin_start(16);
        footer.set_margin_end(16);
        footer.set_margin_top(8);
        footer.set_margin_bottom(8);

        let nav_label = Label::new(Some("↑↓ Navigate"));
        nav_label.add_css_class("footer-label");
        footer.append(&nav_label);

        let select_label = Label::new(Some("↵ Select"));
        select_label.add_css_class("footer-label");
        footer.append(&select_label);

        let esc_label = Label::new(Some("Esc Cancel"));
        esc_label.add_css_class("footer-label");
        esc_label.set_hexpand(true);
        esc_label.set_halign(gtk4::Align::End);
        footer.append(&esc_label);

        main_box.append(&footer);

        window.set_child(Some(&main_box));

        let registry = Rc::new(RefCell::new(registry));
        let results: Rc<RefCell<Vec<SearchResult>>> = Rc::new(RefCell::new(Vec::new()));

        let instance = Self {
            window,
            entry,
            list_box,
            registry,
            results,
        };

        instance.setup_signals();
        instance.load_initial_results();
        instance
    }

    fn setup_signals(&self) {
        // Handle text changes
        let list_box = self.list_box.clone();
        let registry = self.registry.clone();
        let results = self.results.clone();
        let max_results = 10;

        self.entry.connect_changed(move |entry| {
            let query = entry.text().to_string();
            let search_results = registry.borrow().search(&query, max_results);

            // Clear list
            while let Some(child) = list_box.first_child() {
                list_box.remove(&child);
            }

            // Add results
            *results.borrow_mut() = search_results.clone();
            for result in &search_results {
                let row = ResultRow::new(result);
                list_box.append(&row);
            }

            // Select first item
            if let Some(first) = list_box.row_at_index(0) {
                list_box.select_row(Some(&first));
            }
        });

        // Handle Enter key via Entry's activate signal
        let window_for_activate = self.window.clone();
        let list_box_for_activate = self.list_box.clone();
        let entry_for_activate = self.entry.clone();
        let results_for_activate = self.results.clone();
        let registry_for_activate = self.registry.clone();

        self.entry.connect_activate(move |_| {
            if let Some(row) = list_box_for_activate.selected_row() {
                let index = row.index() as usize;
                let result = results_for_activate.borrow().get(index).cloned();
                if let Some(result) = result {
                    if let Err(e) = registry_for_activate.borrow().execute(&result) {
                        log::error!("Failed to execute: {}", e);
                    }
                    window_for_activate.set_visible(false);
                    entry_for_activate.set_text("");
                }
            }
        });

        // Handle keyboard events (Escape, Up, Down)
        let key_controller = EventControllerKey::new();
        let window = self.window.clone();
        let list_box = self.list_box.clone();

        key_controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                gdk::Key::Escape => {
                    window.set_visible(false);
                    glib::Propagation::Stop
                }
                gdk::Key::Down => {
                    if let Some(row) = list_box.selected_row() {
                        let next_index = row.index() + 1;
                        if let Some(next) = list_box.row_at_index(next_index) {
                            list_box.select_row(Some(&next));
                        }
                    }
                    glib::Propagation::Stop
                }
                gdk::Key::Up => {
                    if let Some(row) = list_box.selected_row() {
                        let prev_index = row.index() - 1;
                        if prev_index >= 0 {
                            if let Some(prev) = list_box.row_at_index(prev_index) {
                                list_box.select_row(Some(&prev));
                            }
                        }
                    }
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });

        self.entry.add_controller(key_controller);

        // Handle focus loss - hide window
        let window = self.window.clone();
        self.window.connect_is_active_notify(move |w| {
            if !w.is_active() {
                window.set_visible(false);
            }
        });

        // Handle row activation (double-click)
        let window = self.window.clone();
        let entry = self.entry.clone();
        let results = self.results.clone();
        let registry = self.registry.clone();

        self.list_box.connect_row_activated(move |_, row| {
            let index = row.index() as usize;
            let result = results.borrow().get(index).cloned();
            if let Some(result) = result {
                if let Err(e) = registry.borrow().execute(&result) {
                    log::error!("Failed to execute: {}", e);
                }
                window.set_visible(false);
                entry.set_text("");
            }
        });
    }

    fn load_initial_results(&self) {
        let search_results = self.registry.borrow().search("", 10);
        *self.results.borrow_mut() = search_results.clone();

        for result in &search_results {
            let row = ResultRow::new(result);
            self.list_box.append(&row);
        }

        if let Some(first) = self.list_box.row_at_index(0) {
            self.list_box.select_row(Some(&first));
        }
    }

    pub fn show_window(&self) {
        self.entry.set_text("");
        self.entry.grab_focus();
        self.window.set_visible(true);
        self.window.present();
    }

    pub fn hide_window(&self) {
        self.window.set_visible(false);
    }

    pub fn toggle_window(&self) {
        if self.window.is_visible() {
            self.hide_window();
        } else {
            self.show_window();
        }
    }
}
