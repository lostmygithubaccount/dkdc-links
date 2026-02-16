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

Open links by name or alias:

```bash
# Open a link
dkdc-links link1

# Open multiple links
dkdc-links link1 link2

# Open a group
dkdc-links dev
```

### Options

| Flag | Short | Description |
|------|-------|-------------|
| `--config` | `-c` | Open config file in editor |
| `--app` | `-a` | Open desktop app (requires `app` feature; under construction) |
| `--webapp` | `-w` | Open the webapp (requires `webapp` feature) |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

## Configuration

Config lives at `~/.config/dkdc/links/config.toml`. Run `dkdc-links -c` to edit it.

```toml
[aliases]
gh = "github"
li = "linkedin"

[links]
github = "https://github.com/lostmygithubaccount"
linkedin = "https://linkedin.com/in/codydkdc"

[groups]
social = ["github", "linkedin"]
```

Aliases map to links, links map to URLs, and groups expand to multiple aliases or links.
