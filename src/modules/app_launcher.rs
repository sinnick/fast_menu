use super::{Module, ResultData, SearchResult};
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::process::Command;

#[derive(Debug, Clone)]
struct AppEntry {
    name: String,
    exec: String,
    icon: Option<String>,
    description: Option<String>,
    desktop_file: String,
}

pub struct AppLauncher {
    apps: Vec<AppEntry>,
}

impl AppLauncher {
    pub fn new() -> Self {
        let apps = Self::load_applications();
        Self { apps }
    }

    fn load_applications() -> Vec<AppEntry> {
        let mut apps = Vec::new();
        let locales: &[&str] = &[];

        for path in Iter::new(default_paths()) {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(entry) = DesktopEntry::from_str(&path, &content, Some(locales)) {
                    // Skip hidden entries and entries without Exec
                    if entry.no_display() {
                        continue;
                    }

                    let exec = match entry.exec() {
                        Some(e) => e.to_string(),
                        None => continue,
                    };

                    // Skip terminal-only apps
                    if entry.terminal() {
                        continue;
                    }

                    let name = entry
                        .name(locales)
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    if name.is_empty() {
                        continue;
                    }

                    apps.push(AppEntry {
                        name,
                        exec,
                        icon: entry.icon().map(|s| s.to_string()),
                        description: entry.comment(locales).map(|s| s.to_string()),
                        desktop_file: path.to_string_lossy().to_string(),
                    });
                }
            }
        }

        // Sort by name
        apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        // Remove duplicates by name
        apps.dedup_by(|a, b| a.name.to_lowercase() == b.name.to_lowercase());

        apps
    }
}

impl Module for AppLauncher {
    fn name(&self) -> &str {
        "app_launcher"
    }

    fn search(&self, query: &str, matcher: &SkimMatcherV2) -> Vec<SearchResult> {
        if query.is_empty() {
            return self
                .apps
                .iter()
                .take(20)
                .enumerate()
                .map(|(i, app)| SearchResult {
                    name: app.name.clone(),
                    description: app.description.clone(),
                    icon: app.icon.clone(),
                    module: self.name().to_string(),
                    data: ResultData::App {
                        desktop_file: app.desktop_file.clone(),
                    },
                    score: 1000 - i as i64,
                })
                .collect();
        }

        self.apps
            .iter()
            .filter_map(|app| {
                let score = matcher.fuzzy_match(&app.name, query)?;
                Some(SearchResult {
                    name: app.name.clone(),
                    description: app.description.clone(),
                    icon: app.icon.clone(),
                    module: self.name().to_string(),
                    data: ResultData::App {
                        desktop_file: app.desktop_file.clone(),
                    },
                    score,
                })
            })
            .collect()
    }

    fn execute(&self, result: &SearchResult) -> Result<(), Box<dyn std::error::Error>> {
        if let ResultData::App { desktop_file } = &result.data {
            if let Some(app) = self.apps.iter().find(|a| &a.desktop_file == desktop_file) {
                let exec = app
                    .exec
                    .replace("%u", "")
                    .replace("%U", "")
                    .replace("%f", "")
                    .replace("%F", "")
                    .replace("%i", "")
                    .replace("%c", "")
                    .replace("%k", "");

                let parts: Vec<&str> = exec.split_whitespace().collect();
                if let Some((cmd, args)) = parts.split_first() {
                    Command::new(cmd).args(args).spawn()?;
                }
            }
        }
        Ok(())
    }
}
