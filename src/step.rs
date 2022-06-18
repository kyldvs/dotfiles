use crate::config::Config;
use crate::terminal::Terminal;

pub trait Step {
  fn get_name(&self) -> String;
  fn run(&self, t: &mut Terminal, c: &Config);
}

// Check $PATH variable

pub struct Path;

impl Path {
  pub fn new() -> Path {
    Path
  }
}

impl Step for Path {
  fn get_name(&self) -> String {
    String::from("path")
  }

  fn run(&self, t: &mut Terminal, c: &Config) {
    let name = self.get_name();
    let _ = t.append(format!("  [ ] {}", name));
  }
}
