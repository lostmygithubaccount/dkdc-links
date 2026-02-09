//! CLI for dkdc-links

use anyhow::Result;
use clap::Parser;

use crate::config::{edit_config, init_config, load_config, print_config};
use crate::open::open_links;

#[derive(Parser, Debug)]
#[command(name = "dkdc-links")]
#[command(about = "Bookmarks in your terminal")]
#[command(version)]
pub struct Args {
    #[cfg(feature = "gui")]
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Configure dkdc
    #[arg(short, long)]
    pub config: bool,

    /// Things to open
    pub links: Vec<String>,
}

#[cfg(feature = "gui")]
#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Open the graphical interface
    Gui,
}

/// Run the CLI with the given arguments
pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = Args::parse_from(args);

    // Handle subcommands
    #[cfg(feature = "gui")]
    if let Some(command) = args.command {
        return match command {
            Commands::Gui => crate::gui::run().map_err(|e| anyhow::anyhow!("{e}")),
        };
    }

    // Initialize config (creates default if doesn't exist)
    init_config()?;

    // Handle --config flag
    if args.config {
        edit_config()?;
        return Ok(());
    }

    // Load config
    let config = load_config()?;

    // If no arguments, print config
    if args.links.is_empty() {
        print_config(&config);
    } else {
        // Open the links
        open_links(&args.links, &config)?;
    }

    Ok(())
}
