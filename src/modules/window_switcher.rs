use super::{Module, ResultData, SearchResult};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Clone, Deserialize)]
struct WindowInfo {
    id: u64,
    title: String,
    wm_class: String,
    #[serde(default)]
    workspace: i32,
    #[serde(default)]
    focus: bool,
}

pub struct WindowSwitcher;

impl WindowSwitcher {
    pub fn new() -> Self {
        Self
    }

    fn get_windows(&self) -> Vec<WindowInfo> {
        let output = Command::new("gdbus")
            .args([
                "call",
                "--session",
                "--dest",
                "org.gnome.Shell",
                "--object-path",
                "/org/gnome/Shell/Extensions/Windows",
                "--method",
                "org.gnome.Shell.Extensions.Windows.List",
            ])
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                self.parse_window_list(&stdout)
            }
            Err(e) => {
                log::debug!("Failed to get windows: {}", e);
                Vec::new()
            }
        }
    }

    fn parse_window_list(&self, output: &str) -> Vec<WindowInfo> {
        // Output format: ('[{"id": 123, "title": "...", "wm_class": "..."}]',)
        // We need to extract the JSON array from the GVariant tuple

        let trimmed = output.trim();

        // Remove the outer parentheses and quotes: ('...',) -> ...
        let json_str = if trimmed.starts_with("('") && trimmed.ends_with("',)") {
            &trimmed[2..trimmed.len() - 3]
        } else if trimmed.starts_with("(\"") && trimmed.ends_with("\",)") {
            &trimmed[2..trimmed.len() - 3]
        } else {
            log::debug!("Unexpected output format: {}", trimmed);
            return Vec::new();
        };

        // Unescape the JSON string (replace \\n with \n, etc.)
        let unescaped = json_str
            .replace("\\\"", "\"")
            .replace("\\n", "\n")
            .replace("\\\\", "\\");

        match serde_json::from_str::<Vec<WindowInfo>>(&unescaped) {
            Ok(windows) => windows,
            Err(e) => {
                log::debug!("Failed to parse windows JSON: {} - input: {}", e, unescaped);
                Vec::new()
            }
        }
    }
}

impl Module for WindowSwitcher {
    fn name(&self) -> &str {
        "window_switcher"
    }

    fn search(&self, query: &str, matcher: &SkimMatcherV2) -> Vec<SearchResult> {
        let windows = self.get_windows();

        // Filter out fast-menu window itself
        let windows: Vec<_> = windows
            .into_iter()
            .filter(|w| {
                !w.title.contains("Fast Menu")
                    && !w.wm_class.to_lowercase().contains("fast-menu")
                    && !w.wm_class.to_lowercase().contains("fastmenu")
            })
            .collect();

        if query.is_empty() {
            // Default view: show all windows sorted by focus (focused first)
            return windows
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    let score = if w.focus { 3000 } else { 2000 - i as i64 };
                    SearchResult {
                        name: if w.title.is_empty() {
                            w.wm_class.clone()
                        } else {
                            w.title.clone()
                        },
                        description: Some(format!("{} • Workspace {}", w.wm_class, w.workspace + 1)),
                        icon: Some(w.wm_class.to_lowercase()),
                        module: self.name().to_string(),
                        data: ResultData::Window { window_id: w.id },
                        score,
                    }
                })
                .collect();
        }

        // Fuzzy search on title and wm_class
        windows
            .iter()
            .filter_map(|w| {
                let search_text = format!("{} {}", w.title, w.wm_class);
                let score = matcher.fuzzy_match(&search_text, query)?;

                Some(SearchResult {
                    name: if w.title.is_empty() {
                        w.wm_class.clone()
                    } else {
                        w.title.clone()
                    },
                    description: Some(format!("{} • Workspace {}", w.wm_class, w.workspace + 1)),
                    icon: Some(w.wm_class.to_lowercase()),
                    module: self.name().to_string(),
                    data: ResultData::Window { window_id: w.id },
                    score,
                })
            })
            .collect()
    }

    fn execute(&self, result: &SearchResult) -> Result<(), Box<dyn std::error::Error>> {
        if let ResultData::Window { window_id } = &result.data {
            let output = Command::new("gdbus")
                .args([
                    "call",
                    "--session",
                    "--dest",
                    "org.gnome.Shell",
                    "--object-path",
                    "/org/gnome/Shell/Extensions/Windows",
                    "--method",
                    "org.gnome.Shell.Extensions.Windows.Activate",
                    &window_id.to_string(),
                ])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::error!("Failed to activate window {}: {}", window_id, stderr);
            }
        }
        Ok(())
    }
}
