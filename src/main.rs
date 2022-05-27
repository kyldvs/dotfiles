use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String,

  #[clap(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn main() -> Result<()> {
  let args = Cli::parse();
  let path = &args.path;
  let content = std::fs::read_to_string(path)
    .with_context(|| format!("Could not read file `{}`", path.display()))?;

  for line in content.lines() {
    if line.contains(&args.pattern) {
      println!("{}", line);
    }
  }

  Ok(())
}
