# Read up on just here: https://github.com/casey/just

set fallback := true
set shell := ["bash", "-uc"]
set windows-shell := ["sh", "-uc"]
set ignore-comments

_default:
  @just --list

# Print help.
help:
  @just --list

# =============================================================================
# Simulate sub-commands by invoking with the directory and justfile set.
# =============================================================================

# See: https://github.com/casey/just/issues/208#issuecomment-453529888

# Tasks for the repo itself, not dotfile management.
[group("repo")]
repo +args="help":
  @just -d `pwd` -f "tasks/repo/justfile" {{ args }}

# =============================================================================
# Dotfiles management.
# =============================================================================

home_dir := env_var('HOME')

@_check_program name:
  command -v {{name}} >/dev/null 2>&1 || { echo >&2 "{{name}} is required but it's not installed. Aborting."; exit 1; }

@_check_install:
  just _check_program cat
  just _check_program grep
  just _check_program brew
  just _check_program xargs

@_check_link:
  just _check_install
  just _check_program ls
  just _check_program stow

# Install recipes using brew.
install:
  @just _check_install
  cat ./src/homebrew/recipes.txt | grep -v "^#" | grep -v "^[[:space:]]*$" | xargs brew install

# Links dotfiles into home directory.
link:
  ls -A links | grep -v "^\.\+$" | xargs stow -d ./links -t "{{home_dir}}"

# [confirm("Symlink contents of ./links in home directory? (y/N)")]
# Links dotfiles into test directory (./links-test).
[group("test")]
link-test:
  ls -A links | grep -v "^\.\+$" | xargs stow -d ./links -t ./links-test

# Export installed brew commands. Useful when leaving a system.
[group("leaving")]
@export-brew:
  brew leaves

# =============================================================================
# Manual scripts while getting things working. Should move to better places.
# =============================================================================

# Install fonts.
fonts:
  # Do you even need --cask?
  brew install font-jetbrains-mono-nerd-font
  # Why does this need sudo? Remove that...
  brew install font-sf-pro

# Setup Sketchy bar.
sketchybar:
  brew tap FelixKratz/formulae
  # TODO: Check "Displays have separate Spaces" setting.
  brew install sketchybar
