use super::{Module, ResultData, SearchResult};
use crate::config::FileSearchConfig;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::path::PathBuf;
use std::process::Command;

pub struct FileSearchModule {
    config: FileSearchConfig,
}

impl FileSearchModule {
    pub fn new(config: FileSearchConfig) -> Self {
        Self { config }
    }

    fn expand_path(path: &str) -> PathBuf {
        if path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                return PathBuf::from(path.replacen("~", home.to_str().unwrap_or(""), 1));
            }
        }
        PathBuf::from(path)
    }

    fn search_files(&self, query: &str) -> Vec<PathBuf> {
        let mut results = Vec::new();

        // Try fd first (faster), then fall back to find
        for dir in &self.config.directories {
            let expanded = Self::expand_path(dir);
            if !expanded.exists() {
                continue;
            }

            // Try fd
            if let Ok(output) = Command::new("fd")
                .args([
                    "--max-depth",
                    &self.config.max_depth.to_string(),
                    "--type",
                    "f",
                    "--hidden",
                    "--no-ignore",
                    query,
                ])
                .current_dir(&expanded)
                .output()
            {
                if output.status.success() {
                    for line in String::from_utf8_lossy(&output.stdout).lines() {
                        let path = expanded.join(line);
                        if results.len() < 20 {
                            results.push(path);
                        }
                    }
                    continue;
                }
            }

            // Fallback to find
            if let Ok(output) = Command::new("find")
                .args([
                    expanded.to_str().unwrap_or("."),
                    "-maxdepth",
                    &self.config.max_depth.to_string(),
                    "-type",
                    "f",
                    "-iname",
                    &format!("*{}*", query),
                ])
                .output()
            {
                if output.status.success() {
                    for line in String::from_utf8_lossy(&output.stdout).lines() {
                        if results.len() < 20 {
                            results.push(PathBuf::from(line));
                        }
                    }
                }
            }
        }

        results
    }
}

impl Module for FileSearchModule {
    fn name(&self) -> &str {
        "file_search"
    }

    fn search(&self, query: &str, matcher: &SkimMatcherV2) -> Vec<SearchResult> {
        // Only search files when query has at least 2 characters
        if query.len() < 2 {
            return Vec::new();
        }

        let files = self.search_files(query);

        files
            .into_iter()
            .filter_map(|path| {
                let file_name = path.file_name()?.to_str()?;
                let score = matcher.fuzzy_match(file_name, query).unwrap_or(0);

                Some(SearchResult {
                    name: file_name.to_string(),
                    description: Some(path.parent()?.to_string_lossy().to_string()),
                    icon: Some("text-x-generic".to_string()),
                    module: self.name().to_string(),
                    data: ResultData::File {
                        path: path.to_string_lossy().to_string(),
                    },
                    score: score - 200, // Lower priority than apps
                })
            })
            .collect()
    }

    fn execute(&self, result: &SearchResult) -> Result<(), Box<dyn std::error::Error>> {
        if let ResultData::File { path } = &result.data {
            Command::new("xdg-open").arg(path).spawn()?;
        }
        Ok(())
    }
}
