use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub aliases: HashMap<String, String>,
    #[serde(default)]
    pub links: HashMap<String, String>,
}

const DEFAULT_CONFIG: &str = r#"# dkdc-links config file
[aliases]
alias1 = "link1"
a1 = "link1"
alias2 = "link2"
a2 = "link2"
[links]
link1 = "https://crates.io/crates/dkdc-links"
link2 = "https://github.com/lostmygithubaccount/dkdc-links"
"#;

pub fn config_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME").context("Failed to get HOME environment variable")?;
    Ok(PathBuf::from(home_dir)
        .join(".config")
        .join("dkdc")
        .join("links")
        .join("config.toml"))
}

pub fn init_config() -> Result<()> {
    let config_path = config_path()?;

    if !config_path.exists() {
        let config_dir = config_path.parent().unwrap();
        fs::create_dir_all(config_dir).context("Failed to create config directory")?;
        fs::write(&config_path, DEFAULT_CONFIG).context("Failed to write default config")?;
    }

    Ok(())
}

pub fn load_config() -> Result<Config> {
    let config_path = config_path()?;
    let contents = fs::read_to_string(&config_path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;
    Ok(config)
}

pub fn config_it() -> Result<()> {
    let config_path = config_path()?;
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    println!("Opening {} with {}...", config_path.display(), editor);

    let status = std::process::Command::new(&editor)
        .arg(&config_path)
        .status()
        .with_context(|| format!("Editor {editor} not found in PATH"))?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    Ok(())
}

pub fn print_config(config: &Config) -> Result<()> {
    if !config.aliases.is_empty() {
        println!("aliases:");
        println!();

        let mut keys: Vec<_> = config.aliases.keys().collect();
        keys.sort_unstable();

        let max_key_len = keys.iter().map(|k| k.len()).max().unwrap_or(0);

        for key in keys {
            if let Some(value) = config.aliases.get(key) {
                println!("• {key:<max_key_len$} | {value}");
            }
        }

        println!();
    }

    if !config.links.is_empty() {
        println!("links:");
        println!();

        let mut keys: Vec<_> = config.links.keys().collect();
        keys.sort_unstable();

        let max_key_len = keys.iter().map(|k| k.len()).max().unwrap_or(0);

        for key in keys {
            if let Some(value) = config.links.get(key) {
                println!("• {key:<max_key_len$} | {value}");
            }
        }

        println!();
    }

    Ok(())
}
