pub mod cli;
pub mod config;
#[cfg(feature = "gui")]
pub mod gui;
pub mod open;

// Re-export run from cli module for convenience
pub use cli::run;
