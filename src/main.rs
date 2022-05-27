use anyhow::Result;
use clap::{Parser, Subcommand};
use dotfiles as lib;

#[derive(Parser)]
struct Cli {
  #[clap(subcommand)]
  action: Action,
}

#[derive(Subcommand)]
enum Action {
  Check,
}

fn main() -> Result<()> {
  let args = Cli::parse();

  let action = &args.action;
  match action {
    | Action::Check => lib::check(),
  };

  Ok(())
}
