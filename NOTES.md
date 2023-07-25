# Notes

# Next things to do:

- Consider disable_steps when deciding steps to run.
- Implement "path" step.
- Add "replace_dirs = true" so that windows can rewrite everything in "dirs"

# Rust modules

- https://www.sheshbabu.com/posts/rust-module-system/

# Mac Defaults

- https://macos-defaults.com/

```zsh
# Always show extensions.
defaults write NSGlobalDomain "AppleShowAllExtensions" -bool "true"

# Show hidden files.
defaults write com.apple.Finder "AppleShowAllFiles" -bool "true"

# Don't warn when changing file extensions.
defaults write com.apple.finder "FXEnableExtensionChangeWarning" -bool "false"

# Don't re-arrange spaces.
defaults write com.apple.dock "mru-spaces" -bool "false"

# Dock on the left.
defaults write com.apple.dock "orientation" -string "left"

# Auto-hide the dock.
defaults write com.apple.dock "autohide" -bool "true"
```

# Starship

Cross platform prompt.

- https://starship.rs/

# Scoop

Windows installer of things.

(or chocolatey)

# TUI Design notes

```bash
$ dotfiles
Reading config:
  ~/.config/dotfiles/config.toml ... done
Running steps:
  [1/6] welcome ....... done
  [2/6] path .......... done
  [3/6] fonts ......... needs user input
  [4/6] settings ......
  [5/6] commands ......
  [6/6] programs ......
Step fonts:
  Install fonts using `brew`? [Y/n]:
```
