# git-view

A terminal-based Git repository browser with inline and side-by-side diff views.

## Features

- **Status** — View staged and unstaged file changes
- **Log** — Browse commit history with author, timestamp, and summary
- **Stashes** — List and inspect stashed changes
- **Diff View** — Review diffs in inline or side-by-side mode, with syntax highlighting
- **Customizable Themes** — Light and dark built-in themes, plus RON-based theme files
- **Keyboard-driven** — Vim-style navigation with `j`/`k`, tab switching, and quick-jump keys

## Keybindings

| Key | Action |
|---|---|
| `q` | Quit |
| `h` | Toggle help |
| `d` | Toggle diff view |
| `m` | Toggle inline / side-by-side diff mode |
| `Tab` / `←` `→` | Switch tabs |
| `1` `2` `3` | Go to Status / Log / Stashes tab |
| `↑` `↓` / `k` `j` | Navigate list / scroll diff |
| `Enter` | Show file list for commit/stash, or show diff for file |
| `Esc` | Go back |

## Usage

```bash
# Open the repository in the current directory
git-view

# Open a specific repository
git-view /path/to/repo

# Use a custom theme file
git-view --theme my_theme.ron
```

## Installation

### From source

```bash
git clone https://github.com/lyj-514328/git-view.git
cd git-view
cargo install --path .
```

## Configuration

Themes can be customized via RON files. See the built-in light and dark themes in [src/theme.rs](src/theme.rs) for available fields.

```bash
git-view --theme ~/.config/git-view/theme.ron
```

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [git2](https://github.com/rust-lang/git2-rs) — libgit2 bindings
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [syntect](https://github.com/trishume/syntect) — Syntax highlighting
- [chrono](https://github.com/chronotope/chrono) — Date/time formatting

## License

MIT