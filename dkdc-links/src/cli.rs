use anyhow::Result;
use clap::Parser;

use crate::config::{edit_config, init_config, load_config, print_config};
use crate::open::open_links;

#[derive(Parser, Debug)]
#[command(name = "dkdc-links")]
#[command(about = "Bookmarks in your terminal")]
#[command(version)]
pub struct Args {
    /// Configure dkdc
    #[arg(short, long)]
    pub config: bool,

    /// Open the graphical interface
    #[cfg(feature = "gui")]
    #[arg(long)]
    pub gui: bool,

    /// Things to open
    pub links: Vec<String>,
}

pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = Args::parse_from(args);

    #[cfg(feature = "gui")]
    if args.gui {
        return crate::gui::run().map_err(|e| anyhow::anyhow!("{e}"));
    }

    init_config()?;

    if args.config {
        edit_config()?;
        return Ok(());
    }

    let config = load_config()?;

    if args.links.is_empty() {
        print_config(&config);
    } else {
        open_links(&args.links, &config)?;
    }

    Ok(())
}
