# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

dkdc-links is a Rust CLI tool for managing terminal bookmarks. Users define aliases, links, and groups in a TOML config file, then open them via command line.

## Commands

```bash
# Development
cargo build                    # Build debug binary
cargo run                      # Run the CLI
cargo run -- link1 alias1      # Open specific links/aliases

# Quality checks
./bin/check.sh                 # Format, clippy, and check
cargo fmt                      # Format code
cargo clippy                   # Lint

# Testing
./bin/test.sh                  # Run tests and build release
cargo test --all-features      # Run tests only

# Release
./bin/release.sh               # Clean build and publish to crates.io
```

## Architecture

- `src/main.rs` - CLI entry point using clap for argument parsing
- `src/config.rs` - Config loading/saving from `~/.config/dkdc/links/config.toml`
- `src/open.rs` - Link resolution (alias → link → URI) and opening via system browser

Config structure: aliases map to links, links map to URIs, groups expand to multiple aliases/links.
