pub mod app_launcher;
pub mod commands;
pub mod file_search;
pub mod window_switcher;

use crate::config::Config;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub module: String,
    pub data: ResultData,
    pub score: i64,
}

#[derive(Debug, Clone)]
pub enum ResultData {
    App { desktop_file: String },
    File { path: String },
    Command { command: String },
    Window { window_id: u64 },
}

pub trait Module: Send {
    fn name(&self) -> &str;
    fn search(&self, query: &str, matcher: &SkimMatcherV2) -> Vec<SearchResult>;
    fn execute(&self, result: &SearchResult) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct ModuleRegistry {
    modules: Vec<Box<dyn Module>>,
    matcher: SkimMatcherV2,
}

impl ModuleRegistry {
    pub fn new(config: &Config) -> Self {
        let mut modules: Vec<Box<dyn Module>> = Vec::new();

        // Window switcher FIRST - appears by default when menu opens
        if config.modules.window_switcher {
            modules.push(Box::new(window_switcher::WindowSwitcher::new()));
        }

        if config.modules.app_launcher {
            modules.push(Box::new(app_launcher::AppLauncher::new()));
        }

        if config.modules.commands {
            modules.push(Box::new(commands::CommandsModule::new(
                config.commands.clone(),
            )));
        }

        if config.modules.file_search {
            modules.push(Box::new(file_search::FileSearchModule::new(
                config.file_search.clone(),
            )));
        }

        Self {
            modules,
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn search(&self, query: &str, max_results: usize) -> Vec<SearchResult> {
        let mut all_results: Vec<SearchResult> = Vec::new();

        for module in &self.modules {
            // ONLY use window_switcher - this is a window switcher app
            if module.name() != "window_switcher" {
                continue;
            }
            let results = module.search(query, &self.matcher);
            all_results.extend(results);
        }

        // Sort by score (descending)
        all_results.sort_by(|a, b| b.score.cmp(&a.score));

        // Limit results
        all_results.truncate(max_results);

        all_results
    }

    pub fn execute(&self, result: &SearchResult) -> Result<(), Box<dyn std::error::Error>> {
        for module in &self.modules {
            if module.name() == result.module {
                return module.execute(result);
            }
        }
        Err("Module not found".into())
    }
}
