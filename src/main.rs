use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use dirs::home_dir;

#[derive(Parser)]
struct Cli {
  /// Path to "config.toml" that should be used. Refer to examples for
  /// available settings to use in this file.
  #[clap(short, long)]
  config: Option<PathBuf>,
}

fn main() -> Result<()> {
  let args = Cli::parse();

  // Get the appropriate config file path.
  let home_dir = home_dir().unwrap_or(PathBuf::from("~"));
  let config_rel: PathBuf =
    [".config", "dotfiles", "config.toml"].iter().collect();
  let config_default: PathBuf = [home_dir, config_rel].iter().collect();
  let config = args.config.unwrap_or(config_default);

  dotfiles::run(config);
  Ok(())
}
