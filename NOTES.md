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
