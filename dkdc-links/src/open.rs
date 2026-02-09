use anyhow::{Context, Result};

use crate::config::Config;

pub fn alias_or_link_to_uri(link: &str, config: &Config) -> Result<String> {
    // Check if it's an alias first
    if let Some(alias_target) = config.aliases.get(link) {
        // Alias exists - its target MUST be in links
        return config.links.get(alias_target).cloned().with_context(|| {
            format!("alias '{link}' points to '{alias_target}' which is not in [links]")
        });
    }

    // Check if it's directly in links
    if let Some(uri) = config.links.get(link) {
        return Ok(uri.clone());
    }

    anyhow::bail!("'{}' not found in [aliases] or [links]", link)
}

fn open_it(link: &str) -> Result<()> {
    open::that(link).with_context(|| format!("failed to open {link}"))?;
    println!("opening {link}...");
    Ok(())
}

pub fn expand_groups(links: &[String], config: &Config) -> Vec<String> {
    let mut expanded = Vec::new();
    for link in links {
        if let Some(group_items) = config.groups.get(link) {
            expanded.extend(group_items.clone());
        } else {
            expanded.push(link.clone());
        }
    }
    expanded
}

pub fn open_links(links: &[String], config: &Config) -> Result<()> {
    let expanded_links = expand_groups(links, config);

    for link in expanded_links {
        match alias_or_link_to_uri(&link, config) {
            Ok(uri) => {
                if let Err(e) = open_it(&uri) {
                    eprintln!("[dkdc] failed to open {link}: {e}");
                }
            }
            Err(e) => {
                eprintln!("[dkdc] skipping {link}: {e}");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_config() -> Config {
        let mut aliases = HashMap::new();
        aliases.insert("gh".to_string(), "github".to_string());
        aliases.insert("g".to_string(), "google".to_string());

        let mut links = HashMap::new();
        links.insert("github".to_string(), "https://github.com".to_string());
        links.insert("google".to_string(), "https://google.com".to_string());
        links.insert("rust".to_string(), "https://rust-lang.org".to_string());

        let mut groups = HashMap::new();
        groups.insert(
            "dev".to_string(),
            vec!["gh".to_string(), "rust".to_string()],
        );

        Config {
            aliases,
            links,
            groups,
        }
    }

    #[test]
    fn test_alias_resolves_to_uri() {
        let config = test_config();
        let uri = alias_or_link_to_uri("gh", &config).unwrap();
        assert_eq!(uri, "https://github.com");
    }

    #[test]
    fn test_link_resolves_to_uri() {
        let config = test_config();
        let uri = alias_or_link_to_uri("rust", &config).unwrap();
        assert_eq!(uri, "https://rust-lang.org");
    }

    #[test]
    fn test_alias_target_as_link_resolves() {
        let config = test_config();
        // "github" is both an alias target and a link name
        let uri = alias_or_link_to_uri("github", &config).unwrap();
        assert_eq!(uri, "https://github.com");
    }

    #[test]
    fn test_unknown_link_errors() {
        let config = test_config();
        let result = alias_or_link_to_uri("unknown", &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_alias_to_missing_link_errors() {
        let mut config = test_config();
        config
            .aliases
            .insert("broken".to_string(), "nonexistent".to_string());

        let result = alias_or_link_to_uri("broken", &config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("broken"));
        assert!(err.contains("nonexistent"));
    }

    #[test]
    fn test_expand_group() {
        let config = test_config();
        let links = vec!["dev".to_string()];
        let expanded = expand_groups(&links, &config);
        assert_eq!(expanded, vec!["gh", "rust"]);
    }

    #[test]
    fn test_mixed_groups_and_links() {
        let config = test_config();
        let links = vec!["dev".to_string(), "google".to_string()];
        let expanded = expand_groups(&links, &config);
        assert_eq!(expanded, vec!["gh", "rust", "google"]);
    }
}
