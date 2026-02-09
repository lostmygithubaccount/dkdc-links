use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const DEFAULT_EDITOR: &str = "vi";
const CONFIG_DIR: &str = ".config";
const APP_NAME: &str = "dkdc";
const APP_SUBDIR: &str = "links";
const CONFIG_FILENAME: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub aliases: HashMap<String, String>,
    #[serde(default)]
    pub links: HashMap<String, String>,
    #[serde(default)]
    pub groups: HashMap<String, Vec<String>>,
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
[groups]
dev = ["alias1", "alias2"]
"#;

pub fn config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Failed to get home directory")?;
    Ok(home
        .join(CONFIG_DIR)
        .join(APP_NAME)
        .join(APP_SUBDIR)
        .join(CONFIG_FILENAME))
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

pub fn edit_config() -> Result<()> {
    let config_path = config_path()?;
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| DEFAULT_EDITOR.to_string());

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

fn print_section<V, F>(name: &str, map: &HashMap<String, V>, format_value: F)
where
    F: Fn(&V) -> String,
{
    if map.is_empty() {
        return;
    }

    println!("{name}:");
    println!();

    let mut keys: Vec<_> = map.keys().collect();
    keys.sort_unstable();

    let max_key_len = keys.iter().map(|k| k.len()).max().unwrap_or(0);

    for key in keys {
        if let Some(value) = map.get(key) {
            println!("• {key:<max_key_len$} | {}", format_value(value));
        }
    }

    println!();
}

pub fn print_config(config: &Config) {
    print_section("aliases", &config.aliases, |v| v.clone());
    print_section("links", &config.links, |v| v.clone());
    print_section("groups", &config.groups, |v| format!("[{}]", v.join(", ")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_config() {
        let toml = r#"
[aliases]
gh = "github"

[links]
github = "https://github.com"

[groups]
dev = ["gh"]
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.aliases.get("gh"), Some(&"github".to_string()));
        assert_eq!(
            config.links.get("github"),
            Some(&"https://github.com".to_string())
        );
        assert_eq!(config.groups.get("dev"), Some(&vec!["gh".to_string()]));
    }

    #[test]
    fn test_parse_empty_config() {
        let toml = "";
        let config: Config = toml::from_str(toml).unwrap();
        assert!(config.aliases.is_empty());
        assert!(config.links.is_empty());
        assert!(config.groups.is_empty());
    }

    #[test]
    fn test_parse_partial_config() {
        let toml = r#"
[links]
rust = "https://rust-lang.org"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert!(config.aliases.is_empty());
        assert_eq!(
            config.links.get("rust"),
            Some(&"https://rust-lang.org".to_string())
        );
        assert!(config.groups.is_empty());
    }

    #[test]
    fn test_config_roundtrip() {
        let mut config = Config::default();
        config.aliases.insert("a".to_string(), "b".to_string());
        config
            .links
            .insert("b".to_string(), "https://example.com".to_string());
        config.groups.insert("g".to_string(), vec!["a".to_string()]);

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(config.aliases, deserialized.aliases);
        assert_eq!(config.links, deserialized.links);
        assert_eq!(config.groups, deserialized.groups);
    }

    #[test]
    fn test_default_config_parses() {
        let config: Config = toml::from_str(DEFAULT_CONFIG).unwrap();
        assert!(!config.aliases.is_empty());
        assert!(!config.links.is_empty());
        assert!(!config.groups.is_empty());
    }
}
