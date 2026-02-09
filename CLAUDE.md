# dkdc-links

Bookmarks in your terminal.

## Commands

```bash
bin/build          # Build all (Rust + Python)
bin/build-rs       # Build Rust crate
bin/build-py       # Build Python bindings (maturin develop)
bin/check          # Run all checks (format, lint, test)
bin/check-rs       # Rust checks (fmt, clippy, test)
bin/check-py       # Python checks (ruff)
bin/test           # Run all tests
bin/test-rs        # Rust tests
bin/format         # Format all code
bin/install        # Install CLI (Rust + Python)
```

## Architecture

```
dkdc-links/        # Core Rust crate (standalone, not in monorepo workspace)
  src/lib.rs       # Library root
  src/main.rs      # Binary entry point
  src/cli.rs       # CLI (clap), optional `gui` subcommand
  src/config.rs    # Config loading/saving (~/.config/dkdc/links/config.toml)
  src/open.rs      # Link resolution (alias → link → URI)
  src/gui.rs       # iced GUI (behind `gui` feature flag)
dkdc-links-py/     # PyO3 bindings (cdylib)
src/dkdc_links/    # Python wrapper
```

Config structure: aliases map to links, links map to URIs, groups expand to multiple aliases/links.
