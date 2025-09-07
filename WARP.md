# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**dkdc-links** is a command-line bookmark management tool written in Rust. It allows users to create aliases and links that can be opened from the terminal. The tool provides a simple TOML-based configuration system and integrates with the system's default application opener.

## Architecture

The codebase follows a simple modular structure:

- **`main.rs`**: Entry point with CLI argument parsing using clap
- **`config.rs`**: Configuration management (TOML parsing, file I/O, default config)
- **`open.rs`**: Link resolution and opening logic
- **`lib.rs`**: Module declarations

### Key Components

**Configuration System**: Uses a two-level lookup system where aliases can point to link names, and links contain the actual URLs. Config is stored at `~/.config/dkdc/links/config.toml`.

**Link Resolution**: The `alias_or_link_to_uri` function implements the core logic:
1. Check if input is an alias → resolve to link name → get URL
2. If not an alias, check if input is directly a link name → get URL
3. Return error if neither found

## Development Commands

### Building and Testing
```bash
# Format, lint, and check code
./bin/check.sh

# Run tests and build release binary
./bin/test.sh

# Build for development
cargo build

# Build optimized release
cargo build --release
```

### Running Locally
```bash
# Run with development build
cargo run

# Run with arguments (view help)
cargo run -- --help

# Test opening links
cargo run -- alias1 link1

# Test configuration editing
cargo run -- --config
```

### Publishing
```bash
# Clean build and publish to crates.io
./bin/release.sh
```

## Configuration Details

The tool expects a TOML configuration file with two sections:
- `[aliases]`: Maps short names to link names
- `[links]`: Maps link names to actual URLs

Example structure:
```toml
[aliases]
gh = "github"
[links]  
github = "https://github.com/lostmygithubaccount/dkdc-links"
```

## Error Handling

The codebase uses `anyhow::Result` throughout for error handling. Key error scenarios:
- Missing config file (auto-creates default)
- Invalid TOML syntax
- Links/aliases not found
- Failed system calls (opening links, editor)
