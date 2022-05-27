mod config;
mod out;
mod utils;

use core::slice::Iter;
use std::cmp::max;

use colored::Colorize;
use out::out;

fn get_max_len(items: Iter<&str>) -> usize {
  let len = items.fold(0, |acc, &c| max(acc, c.len()));
  return len;
}

fn check_path() {
  let common = vec![
    "/bin",
    "/sbin",
    "/usr/bin",
    "/usr/sbin",
    "/usr/local/bin",
    "/usr/local/sbin",
  ];

  let mac_default = vec!["*/homebrew/bin"];
  let mac = if utils::is_mac() { mac_default } else { vec![] };

  let rust = vec!["*/.cargo/bin"];

  let expected = vec![common, rust, mac].concat();
  let mut missing: Vec<&str> = vec![];

  let path = std::env::var("PATH");

  match path {
    | Err(_) => out(2, "Error getting $PATH, maybe it's not set?".red()),
    | Ok(path) => {
      let parts: Vec<&str> = path.split(":").collect();
      for item in expected.iter() {
        let mut exists = false;
        for part in parts.iter() {
          // If the item starts with a wildcard, we only are looking for a
          // path item to end with this pattern (minus the wildcard).
          if item.starts_with("*") {
            if part.ends_with(&item[1..]) {
              exists = true;
            }
          } else {
            if part == item {
              exists = true;
            }
          }

          if exists {
            break;
          }
        }

        // Put any missing items into the missing vector, used for reporting.
        if !exists {
          missing.push(item);
        }
      }
    },
  };

  let overview = format!(
    "Verified {}/{} expected locations",
    expected.len() - missing.len(),
    expected.len()
  );

  let colored_overview =
    if missing.len() == 0 { overview.green() } else { overview.yellow() };

  out(2, colored_overview);

  let max_len = get_max_len(missing.iter());
  for item in missing {
    out(2, format!("{:max_len$}  {}", item, "Missing".yellow()));
  }
}

fn check_common_commands() {
  let commands = [
    "sudo", "exa", "htop", "git", "hg", "cargo", "rustup", "fd", "ripgrep",
    "vim", "nvim", "nano", "tmux", "nvm",
  ];

  let max_len = get_max_len(commands.iter());

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
    out(1, "Checking $PATH setup".blue());
    check_path();
    out(0, "");
    out(1, "Checking commands".blue());
    check_common_commands();
    out(0, "");
    out(1, "All done, goodbye!".green());
    out(0, "");
  }
}
