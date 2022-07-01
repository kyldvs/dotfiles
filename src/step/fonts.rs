#![allow(dead_code)]

use std::collections::HashSet;

use super::{Fonts, Step};
use crate::config::Config;
use crate::terminal::Terminal;
use crate::utils::path::all_files_recursive;
use crate::utils::platform;

fn get_font_dirs() -> Vec<String> {
  let dirs = if platform::is_mac() {
    vec![
      String::from("~/Library/Fonts"),
      String::from("/Library/Fonts"),
      String::from("/System/Library/Fonts"),
      String::from("/System/Library/Fonts/Supplemental"),
    ]
  } else {
    vec![]
  };
  dirs
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

    let mut installed_fonts = HashSet::new();
    for dir in get_font_dirs() {
      for file in all_files_recursive(dir) {
        // TODO: Nicer way to get font name from path.
        let name = format!("{}", file.file_name().unwrap().to_str().unwrap());
        installed_fonts.insert(name);
      }
    }

    let check = |names: Vec<&str>| {
      for name in names {
        if !installed_fonts.contains(name) {
          return false;
        }
      }
      return true;
    };

    let mut good = 0;
    let mut missing = vec![];
    for font in c.get_fonts() {
      // Handle Fira Code font set
      if font == "Fira Code" {
        // Variable font install. Includes all of the other ones except retina.
        if check(vec!["FiraCode-VF.ttf", "FiraCode-Retina.ttf"]) {
          good = good + 1;
          continue;
        }

        if check(vec![
          "FiraCode-Bold.ttf",
          "FiraCode-Light.ttf",
          "FiraCode-Medium.ttf",
          "FiraCode-Regular.ttf",
          "FiraCode-SemiBold.ttf",
          // Is this one necessary in normal install?
          "FiraCode-Retina.ttf",
        ]) {
          good = good + 1;
          continue;
        }
      }

      if installed_fonts.contains(&font) {
        good = good + 1;
        continue;
      }

      missing.push(font);
    }

    t.append(format!("    Found {} good fonts", good));

    t.update(status_line, format!("  [x] {}", name))
  }
}
