use super::{Module, ResultData, SearchResult};
use crate::config::CustomCommand;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::process::Command;

pub struct CommandsModule {
    commands: Vec<CustomCommand>,
}

impl CommandsModule {
    pub fn new(commands: Vec<CustomCommand>) -> Self {
        Self { commands }
    }
}

impl Module for CommandsModule {
    fn name(&self) -> &str {
        "commands"
    }

    fn search(&self, query: &str, matcher: &SkimMatcherV2) -> Vec<SearchResult> {
        if query.is_empty() {
            return self
                .commands
                .iter()
                .enumerate()
                .map(|(i, cmd)| SearchResult {
                    name: cmd.name.clone(),
                    description: cmd.description.clone(),
                    icon: cmd.icon.clone(),
                    module: self.name().to_string(),
                    data: ResultData::Command {
                        command: cmd.command.clone(),
                    },
                    score: 500 - i as i64, // Lower base score than apps
                })
                .collect();
        }

        self.commands
            .iter()
            .filter_map(|cmd| {
                let score = matcher.fuzzy_match(&cmd.name, query)?;
                Some(SearchResult {
                    name: cmd.name.clone(),
                    description: cmd.description.clone(),
                    icon: cmd.icon.clone(),
                    module: self.name().to_string(),
                    data: ResultData::Command {
                        command: cmd.command.clone(),
                    },
                    score: score - 100, // Slightly lower than apps for same match
                })
            })
            .collect()
    }

    fn execute(&self, result: &SearchResult) -> Result<(), Box<dyn std::error::Error>> {
        if let ResultData::Command { command } = &result.data {
            Command::new("sh").arg("-c").arg(command).spawn()?;
        }
        Ok(())
    }
}
