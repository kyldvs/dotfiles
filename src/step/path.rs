use std::cmp::max;
use std::collections::HashSet;

use super::{Path, Step};
use crate::config::Config;
use crate::terminal::Terminal;
use crate::utils;

impl Path {
  pub fn new() -> Path {
    Path
  }
}

impl Step for Path {
  fn get_name(&self) -> String {
    String::from("path")
  }

  // Ideas: Warn on relative parts of path? i.e. foo/../bar
  fn run(&self, t: &mut Terminal, c: &Config) {
    let name = self.get_name();
    let status_line = t.append(format!("  [ ] {}", name));

    // Try to get the path environment variable. Report any errors.
    let path = std::env::var("PATH");
    if let Err(_) = path {
      t.append("    > Error getting $PATH, maybe it's not set?");
      t.update(status_line, format!("  [x] {}", name));
      return;
    };

    let mut path_parts = HashSet::new();

    // Already handled error, this should be safe.
    let parts = path.unwrap();
    let parts = parts.split(":");
    for part in parts {
      path_parts.insert(part);
    }

    // These are the things we expect to be in the path.
    let expected = c.get_path_dirs();
    let expected_len = expected.len();

    let mut missing: Vec<String> = vec![];
    let mut max_len = 1;

    for item in expected {
      let expanded = utils::path::expand_tilde(&item);
      let expanded = expanded.unwrap();
      let expanded = expanded.to_str();
      let expanded = expanded.unwrap();
      if !path_parts.contains(expanded) {
        max_len = max(max_len, item.len());
        missing.push(item);
      }
    }

    let overview = format!(
      "Found {}/{} expected locations",
      expected_len - missing.len(),
      expected_len
    );

    let colored_overview = if missing.len() == 0 {
      format!(
        "{}{}{}",
        termion::color::Fg(termion::color::Green),
        overview,
        termion::color::Fg(termion::color::Reset)
      )
    } else {
      format!(
        "{}{}{}",
        termion::color::Fg(termion::color::Yellow),
        overview,
        termion::color::Fg(termion::color::Reset)
      )
    };

    t.append(format!("    {}", colored_overview));

    for item in missing {
      t.append(format!("    {:max_len$}  {}", item, "Missing"));
    }

    t.update(status_line, format!("  [x] {}", name))
  }
}
