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

```bash
dkdc-links [OPTIONS] [LINKS]...
```

### Configuration

Configuration lives at `$HOME/.config/dkdc/links/config.toml`. Example:

```toml
[links]
github = "https://github.com"
linkedin = "https://linkedin.com"

[aliases]
gh = "github"
li = "linkedin"

[groups]
social = ["github", "linkedin"]
```

Links map to URLs, aliases map to links, and groups map to a list of aliases or links.

Use the `--config` or `--app` or `--webapp` option to edit the configuration file.

### Open links

Open links by name or alias or group:

```bash
dkdc-links github
dkdc-links gh linkedin
dkdc-links social
```

You can input multiple links, aliases, or groups at once. They will be opened in the order they are provided.

### Options

Available options:

| Flag | Short | Description |
|------|-------|-------------|
| `--config` | `-c` | Open configuration file in `$EDITOR` |
| `--app` | `-a` | Open desktop app (requires `app` feature) |
| `--webapp` | `-w` | Open the web app in browser (requires `webapp` feature) |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

