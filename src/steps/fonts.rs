#![allow(dead_code)]

use colored::Colorize;

use crate::out::out;

const MAC_FONT_DIRS: &'static [&'static str] = &[
  "/Library/Fonts",
  "/System/Library/Fonts",
  "/System/Library/Fonts/Supplemental",
];

fn is_installed(name: String) -> bool {
  return name != "";
}

pub fn run() {
  out(2, "Checking fonts".cyan());
  out(3, "Not yet impletmented.".yellow());
}
