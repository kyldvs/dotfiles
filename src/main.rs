use std::path::PathBuf;

use anyhow::Result;
use clap::{ErrorKind, IntoApp, Parser};
use dotfiles as lib;

#[derive(Parser)]
struct Cli {
  /// Path to "config.toml" that should be used. Refer to examples for
  /// available options.
  #[clap(short, long)]
  config: Option<PathBuf>,
}

fn main() -> Result<()> {
  let args = Cli::parse();

  match args.config {
    | None => {
      let mut command = Cli::command();
      let _ = command
        .error(ErrorKind::EmptyValue, "Missing required --config option.")
        .exit();
    },
    | Some(config) => {
      lib::run(config);
    },
  }

  Ok(())
}
