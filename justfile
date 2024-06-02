# Read up on just here: https://github.com/casey/just

set fallback := true
set shell := ["bash", "-uc"]
set windows-shell := ["sh", "-uc"]

_default:
  @just --list

# Print help.
help:
  @just --list

# =============================================================================
# Simulate sub-commands by invoking with the directory and justfile set.
# =============================================================================

# See: https://github.com/casey/just/issues/208#issuecomment-453529888

# Common chores that need to be done.
chore +args="help":
  @just -d `pwd` -f "tasks/chore/justfile" {{ args }}

# Tasks particular to a CI environment.
ci +args="help":
  @just -d `pwd` -f "tasks/ci/justfile" {{ args }}

# =============================================================================
# Repo management.
# =============================================================================

# Run this first.
[group("repo")]
install:
  pnpm install

[group("repo")]
build:
  echo "There is no build command"

[group("repo")]
test:
  echo "There is no test command"

[group("repo")]
format:
  pnpm exec prettier --write "**/*.{js,jsx,ts,tsx,json,yml,yaml,md,mdx}"
