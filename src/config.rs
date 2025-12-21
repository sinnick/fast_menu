use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub modules: ModulesConfig,
    #[serde(default)]
    pub file_search: FileSearchConfig,
    #[serde(default)]
    pub commands: Vec<CustomCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulesConfig {
    #[serde(default = "default_true")]
    pub window_switcher: bool,
    #[serde(default = "default_true")]
    pub app_launcher: bool,
    #[serde(default = "default_true")]
    pub file_search: bool,
    #[serde(default = "default_true")]
    pub commands: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchConfig {
    #[serde(default = "default_directories")]
    pub directories: Vec<String>,
    #[serde(default = "default_max_depth")]
    pub max_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCommand {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

fn default_theme() -> String {
    "dark".to_string()
}

fn default_max_results() -> usize {
    10
}

fn default_true() -> bool {
    true
}

fn default_directories() -> Vec<String> {
    vec![
        "~".to_string(),
        "~/Documents".to_string(),
        "~/Projects".to_string(),
    ]
}

fn default_max_depth() -> u32 {
    3
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            modules: ModulesConfig::default(),
            file_search: FileSearchConfig::default(),
            commands: vec![
                CustomCommand {
                    name: "Lock Screen".to_string(),
                    command: "loginctl lock-session".to_string(),
                    icon: Some("system-lock-screen".to_string()),
                    description: Some("Lock the screen".to_string()),
                },
                CustomCommand {
                    name: "Suspend".to_string(),
                    command: "systemctl suspend".to_string(),
                    icon: Some("system-suspend".to_string()),
                    description: Some("Suspend the system".to_string()),
                },
                CustomCommand {
                    name: "Shutdown".to_string(),
                    command: "systemctl poweroff".to_string(),
                    icon: Some("system-shutdown".to_string()),
                    description: Some("Shutdown the system".to_string()),
                },
                CustomCommand {
                    name: "Reboot".to_string(),
                    command: "systemctl reboot".to_string(),
                    icon: Some("system-reboot".to_string()),
                    description: Some("Reboot the system".to_string()),
                },
            ],
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            max_results: default_max_results(),
        }
    }
}

impl Default for ModulesConfig {
    fn default() -> Self {
        Self {
            window_switcher: true,
            app_launcher: true,
            file_search: true,
            commands: true,
        }
    }
}

impl Default for FileSearchConfig {
    fn default() -> Self {
        Self {
            directories: default_directories(),
            max_depth: default_max_depth(),
        }
    }
}

impl Config {
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("fast-menu"))
    }

    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("config.toml"))
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path().ok_or("Could not determine config path")?;

        if !path.exists() {
            // Create default config
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dir = Self::config_dir().ok_or("Could not determine config dir")?;
        fs::create_dir_all(&dir)?;

        let path = Self::config_path().ok_or("Could not determine config path")?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn theme_path(&self) -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("themes").join(format!("{}.css", self.general.theme)))
    }

    pub fn custom_css_path() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("theme.css"))
    }
}
