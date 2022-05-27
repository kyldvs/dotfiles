mod config;
mod out;
mod utils;

use std::cmp::max;

use colored::Colorize;
use out::out;

fn check_common_commands() {
  let commands = ["sudo", "exa", "htop"];

  let max_len = commands.iter().fold(0, |acc, &c| max(acc, c.len()));

  for command in commands {
    let exists = utils::has_command(command);
    let result = if exists { "Good".green() } else { "Missing".red() };
    out(2, format!("{:max_len$}  {}", command, result));
  }
}

pub fn check() {
  if utils::is_windows() {
    panic!("Windows is not supported yet.")
  }

  if utils::is_mac() || utils::is_arch_linux() {
    out(0, "");
    out(1, "Checking commands".blue());
    check_common_commands();
    out(0, "");
    out(1, "All done, goodbye!".green());
    out(0, "");
  }
}
