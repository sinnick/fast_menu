mod app;
mod config;
mod modules;
mod search;
mod ui;
mod window;

use clap::{Parser, Subcommand};
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;

const APP_ID: &str = "com.github.fastmenu";

#[derive(Parser)]
#[command(name = "fast-menu")]
#[command(about = "A fast, extensible application launcher for Wayland")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Show the launcher window
    Show,
    /// Hide the launcher window
    Hide,
    /// Toggle the launcher window visibility
    Toggle,
}

fn main() -> glib::ExitCode {
    env_logger::init();


    let cli = Cli::parse();

    // Try to activate existing instance via D-Bus
    if let Some(cmd) = &cli.command {
        if try_send_command(cmd) {
            return glib::ExitCode::SUCCESS;
        }
    }

    // No existing instance, start the application
    let app = app::FastMenuApp::new(APP_ID);
    app.run_with_command(cli.command)
}

/// Try to send command to existing instance via D-Bus
fn try_send_command(cmd: &Commands) -> bool {
    let connection = match gio::bus_get_sync(gio::BusType::Session, gio::Cancellable::NONE) {
        Ok(conn) => conn,
        Err(_) => return false,
    };

    let action = match cmd {
        Commands::Show => "show",
        Commands::Hide => "hide",
        Commands::Toggle => "toggle",
    };

    // Try to activate the action on the remote application
    let result = connection.call_sync(
        Some(APP_ID),
        &format!("/{}", APP_ID.replace('.', "/")),
        "org.gtk.Actions",
        "Activate",
        Some(
            &(
                action,
                Vec::<String>::new(),
                std::collections::HashMap::<String, glib::Variant>::new(),
            )
                .to_variant(),
        ),
        None,
        gio::DBusCallFlags::NONE,
        1000,
        gio::Cancellable::NONE,
    );

    result.is_ok()
}
