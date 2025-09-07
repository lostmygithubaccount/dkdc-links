use anyhow::{Context, Result};

use crate::config::Config;

pub fn alias_or_link_to_uri(link: &str, config: &Config) -> Result<String> {
    // Check if it's an alias first
    if let Some(alias_target) = config.aliases.get(link) {
        // Now check if the alias target is in links
        if let Some(uri) = config.links.get(alias_target) {
            return Ok(uri.clone());
        }
    }

    // Check if it's directly in links
    if let Some(uri) = config.links.get(link) {
        return Ok(uri.clone());
    }

    anyhow::bail!("'{}' not found in [links], [aliases], or [groups]", link)
}

fn open_it(link: &str) -> Result<()> {
    open::that(link).with_context(|| format!("failed to open {link}"))?;
    println!("opening {link}...");
    Ok(())
}

pub fn open_links(links: Vec<String>, config: &Config) -> Result<()> {
    // First, expand any groups in the input
    let mut expanded_links = Vec::new();
    for link in links {
        if let Some(group_items) = config.groups.get(&link) {
            // It's a group, add all items from the group
            expanded_links.extend(group_items.clone());
        } else {
            // Not a group, add the item directly
            expanded_links.push(link);
        }
    }

    // Now process the expanded list
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
