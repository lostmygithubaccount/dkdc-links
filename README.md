# Bookmarks

Bookmarks in your terminal.

## Install

Recommended:

```bash
curl -LsSf https://dkdc.sh/dkdc-links/install.sh | sh
```

uv:

```bash
uv tool install dkdc-links
```

cargo:

```bash
cargo install dkdc-links --features app,webapp
```

You can use `uvx` to run it without installing:

```bash
uvx dkdc-links
```

## Usage

```
dkdc-links [OPTIONS] [LINKS]...
```

Configuration lives at `$HOME/.config/dkdc/links/config.toml`. Example:

```toml
[aliases]
gh = "github"
li = "linkedin"

[links]
github = "https://github.com"
linkedin = "https://linkedin.com"

[groups]
social = ["github", "linkedin"]
```

Open links by name or alias:

```bash
# Open a link
dkdc-links github

# Open multiple links
dkdc-links gh linkedin

# Open a group
dkdc-links social
```

### Options

| Flag | Short | Description |
|------|-------|-------------|
| `--config` | `-c` | Open configuration file in `$EDITOR` |
| `--app` | `-a` | Open desktop app (requires `app` feature) |
| `--webapp` | `-w` | Open the web app in browser (requires `webapp` feature) |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

## Configuration

Aliases map to links, links map to URLs, and groups expand to multiple aliases or links.
