mod fonts;
mod path;

use crate::config::Config;
use crate::terminal::Terminal;

pub trait Step: Sync {
  fn get_name(&self) -> String;
  fn run(&self, t: &mut Terminal, c: &Config);
}

/// Check $PATH variable contains appropriate directories.
pub struct Path;

/// Check that correct fonts are installed.
pub struct Fonts;

pub fn get_steps() -> Vec<Box<dyn Step>> {
  vec![Box::new(Path::new()), Box::new(Fonts::new())]
}
