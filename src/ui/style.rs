use crate::config::Config;
use gtk4::gdk::Display;
use gtk4::CssProvider;

const DEFAULT_CSS: &str = r#"
/* Fast Menu - Terminal UI */

/* Reset and base styles */
* {
    border-radius: 0;
}

/* Window background */
window,
window.background,
.fast-menu {
    background-color: #0c0c0c;
    border: 1px solid #1a8ac1;
}

/* All child elements transparent */
.fast-menu *,
.main-box,
.main-box *,
box {
    background-color: transparent;
    background-image: none;
}

/* Main box container */
.main-box {
    background-color: #0c0c0c;
}

/* Search area */
.search-box {
    background-color: #0c0c0c;
}

.search-icon {
    color: #1a8ac1;
    font-family: monospace;
    font-size: 17px;
}

/* Search entry */
entry,
entry.search-entry,
.search-entry {
    background-color: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    color: #ffffff;
    font-family: monospace;
    font-size: 17px;
    min-height: 0;
    padding: 0;
    caret-color: #1a8ac1;
}

entry > text,
entry.search-entry > text,
.search-entry > text {
    background: none;
    background-color: transparent;
    color: #ffffff;
}

entry:focus,
entry.search-entry:focus,
.search-entry:focus {
    box-shadow: none;
    border: none;
    outline: none;
}

/* Separator lines */
.separator {
    background-color: #1a8ac1;
    min-height: 1px;
}

/* Scrolled window */
scrolledwindow,
scrolledwindow.results-scroll,
.results-scroll {
    background-color: #0c0c0c;
}

scrolledwindow > viewport,
scrolledwindow.results-scroll > viewport {
    background-color: #0c0c0c;
}

scrolledwindow undershoot,
scrolledwindow overshoot,
scrolledwindow.results-scroll undershoot,
scrolledwindow.results-scroll overshoot {
    background: none;
}

/* Results list */
listbox,
listbox.results-list,
.results-list {
    background-color: #0c0c0c;
}

listbox > row,
listbox.results-list > row {
    background-color: #0c0c0c;
    outline: none;
    border: none;
}

listbox > row:selected,
listbox.results-list > row:selected {
    background-color: #1a8ac1;
}

listbox > row:selected *,
listbox.results-list > row:selected * {
    background-color: transparent;
}

listbox > row:selected label,
listbox.results-list > row:selected label {
    color: #0c0c0c;
}

/* Result row content */
.result-row {
    background-color: transparent;
}

.result-left {
    font-family: monospace;
    font-size: 15px;
    font-weight: 500;
    color: #ffffff;
}

.result-right {
    font-family: monospace;
    font-size: 15px;
    color: #1a8ac1;
}

listbox > row:selected .result-left,
listbox.results-list > row:selected .result-left {
    color: #0c0c0c;
}

listbox > row:selected .result-right,
listbox.results-list > row:selected .result-right {
    color: #0c0c0c;
}

/* Footer */
.footer {
    background-color: #0c0c0c;
}

.footer-label {
    font-family: monospace;
    font-size: 13px;
    color: #ffffff;
}
"#;

pub fn load_css() {
    let provider = CssProvider::new();

    // Try to load custom CSS first
    let custom_css_loaded = if let Some(custom_path) = Config::custom_css_path() {
        eprintln!("[CSS] Checking custom CSS path: {:?}", custom_path);
        if custom_path.exists() {
            eprintln!("[CSS] Loading custom CSS from {:?}", custom_path);
            provider.load_from_path(&custom_path);
            true
        } else {
            false
        }
    } else {
        false
    };

    // Try theme CSS
    if !custom_css_loaded {
        if let Ok(config) = Config::load() {
            if let Some(theme_path) = config.theme_path() {
                eprintln!("[CSS] Checking theme path: {:?}", theme_path);
                if theme_path.exists() {
                    eprintln!("[CSS] Loading theme CSS from {:?}", theme_path);
                    provider.load_from_path(&theme_path);
                } else {
                    eprintln!("[CSS] Theme path doesn't exist, loading DEFAULT_CSS");
                    provider.load_from_data(DEFAULT_CSS);
                }
            } else {
                eprintln!("[CSS] No theme path, loading DEFAULT_CSS");
                provider.load_from_data(DEFAULT_CSS);
            }
        } else {
            eprintln!("[CSS] Config load failed, loading DEFAULT_CSS");
            provider.load_from_data(DEFAULT_CSS);
        }
    }

    // Apply CSS with maximum priority
    if let Some(display) = Display::default() {
        eprintln!("[CSS] Applying CSS to display");
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_USER + 100,
        );
    } else {
        eprintln!("[CSS] ERROR: Display::default() returned None - CSS not applied!");
    }
}
