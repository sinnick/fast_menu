use crate::config::Config;
use gtk4::gdk::Display;
use gtk4::CssProvider;

const DEFAULT_CSS: &str = r#"
/* Fast Menu - Terminal UI */

* { border-radius: 0; }

window.background { background-color: #0c0c0c; }
.fast-menu { background-color: #0c0c0c; border: 1px solid #1a8ac1; }
.fast-menu * { background-color: transparent; background-image: none; }

box { background-color: #0c0c0c; }

.search-icon { color: #1a8ac1; font-family: monospace; font-size: 17px; }

entry.search-entry {
    background-color: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    color: #ffffff;
    font-family: monospace;
    font-size: 17px;
    min-height: 0;
    padding: 0;
}
entry.search-entry > text { background: none; color: #ffffff; }
entry.search-entry:focus { box-shadow: none; border: none; outline: none; }

.separator { background-color: #1a8ac1; min-height: 1px; }

scrolledwindow.results-scroll { background-color: #0c0c0c; }
scrolledwindow.results-scroll > viewport { background-color: #0c0c0c; }
scrolledwindow.results-scroll undershoot,
scrolledwindow.results-scroll overshoot { background: none; }

listbox.results-list { background-color: #0c0c0c; }
listbox.results-list > row { background-color: #0c0c0c; outline: none; }
listbox.results-list > row:selected { background-color: #1a8ac1; }
listbox.results-list > row:selected * { background-color: transparent; }
listbox.results-list > row:selected label { color: #0c0c0c; }

.result-left { font-family: monospace; font-size: 15px; font-weight: 500; color: #ffffff; }
.result-right { font-family: monospace; font-size: 15px; color: #1a8ac1; }
listbox.results-list > row:selected .result-left { color: #0c0c0c; }
listbox.results-list > row:selected .result-right { color: #0c0c0c; }

.footer-label { font-family: monospace; font-size: 13px; color: #ffffff; }
"#;

pub fn load_css() {
    let provider = CssProvider::new();

    // Try to load custom CSS first
    let custom_css_loaded = if let Some(custom_path) = Config::custom_css_path() {
        if custom_path.exists() {
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
                if theme_path.exists() {
                    provider.load_from_path(&theme_path);
                } else {
                    provider.load_from_data(DEFAULT_CSS);
                }
            } else {
                provider.load_from_data(DEFAULT_CSS);
            }
        } else {
            provider.load_from_data(DEFAULT_CSS);
        }
    }

    // Apply CSS with maximum priority
    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_USER + 100,
        );
    }
}
