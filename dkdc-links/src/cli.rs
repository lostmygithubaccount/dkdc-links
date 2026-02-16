use anyhow::Result;
use clap::Parser;

use crate::config::{edit_config, print_config};
use crate::open::open_links;
use crate::storage::Storage;
use crate::toml_storage::TomlStorage;

#[derive(Parser, Debug)]
#[command(name = "dkdc-links")]
#[command(about = "Bookmarks in your terminal")]
#[command(version)]
pub struct Args {
    /// Configure dkdc
    #[arg(short, long)]
    pub config: bool,

    /// Open the desktop app
    #[cfg(feature = "app")]
    #[arg(short = 'a', long)]
    pub app: bool,

    /// Open the webapp
    #[cfg(feature = "webapp")]
    #[arg(short = 'w', long)]
    pub webapp: bool,

    /// Things to open
    pub links: Vec<String>,
}

pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = Args::parse_from(args);

    #[cfg(feature = "app")]
    if args.app {
        return crate::app::run().map_err(|e| anyhow::anyhow!("{e}"));
    }

    let storage = TomlStorage::with_default_path()?;

    #[cfg(feature = "webapp")]
    if args.webapp {
        storage.init()?;
        return crate::webapp::run(Box::new(storage));
    }
    storage.init()?;

    if args.config {
        if let Some(path) = storage.path() {
            edit_config(path)?;
        }
        return Ok(());
    }

    let config = storage.load()?;

    if args.links.is_empty() {
        print_config(&config);
    } else {
        open_links(&args.links, &config)?;
    }

    Ok(())
}
