# dotfiles

Some Dotfiles and a tool to get everyting setup.

# Setup

[Install rust toolchain](https://www.rust-lang.org/tools/install)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Run

```
cargo run
```

# Development

Use nightly toolchain for development, mainly for more rustfmt features.

```
rustup update nightly-aarch64-apple-darwin
rustup default nightly
```
