use super::{Fonts, Step};
use crate::config::Config;
use crate::terminal::Terminal;

const MAC_FONT_DIRS: &'static [&'static str] = &[
  "/Library/Fonts",
  "/System/Library/Fonts",
  "/System/Library/Fonts/Supplemental",
];

fn is_installed(name: String) -> bool {
  return name != "";
}

impl Fonts {
  pub fn new() -> Fonts {
    Fonts
  }
}

impl Step for Fonts {
  fn get_name(&self) -> String {
    String::from("fonts")
  }

  fn run(&self, t: &mut Terminal, c: &Config) {
    let name = self.get_name();
    let status_line = t.append(format!("  [ ] {}", name));
    t.append(format!("    not yet implemented"));
    t.update(status_line, format!("  [x] {}", name))
  }
}
