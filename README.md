# dotfiles

[![CI Badge](https://github.com/kyldvs/dotfiles/actions/workflows/ci.yml/badge.svg)](https://github.com/kyldvs/dotfiles/actions/workflows/ci.yml)

Some Dotfiles and a tool to get everyting setup.

## Install Tools

- Install [`cargo`, `rustfmt`, etc.](https://www.rust-lang.org/tools/install)
  - Rust toolchain.
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install [`nvm`](https://github.com/nvm-sh/nvm#installing-and-updating)
  - Manages node version.
- Install [`pnpm`](https://pnpm.io/installation)
  - Package manager.

## Setup Environment

_Run all of these commands once after checking out the repo._

- `nvm use`
  - Picks version of node based on `.nvmrc`.
- `pnpm install`
  - Install relevant packages.
  - Setup git commit hooks from the `.husky` folder.
- `rustup toolchain install nightly`
  - Make sure nightly toolchain is available for extra `rustfmt` options.
- `rustup default nightly`
  - Switch to nightly.

## Run

```
cargo run
```
