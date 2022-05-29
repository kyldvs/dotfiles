use core::slice::Iter;
use std::cmp::max;

use colored::Colorize;
use toml::Value;

use crate::config::parse::{Config, Path, PathPlatform, Step};
use crate::out::out;
use crate::utils;

fn get_max_len(items: Iter<&String>) -> usize {
  let len = items.fold(0, |acc, c| max(acc, c.len()));
  return len;
}

fn get_path_dirs(config: &Config) -> Vec<String> {
  let step: Option<&Step> = config.step.as_ref();
  let path: Option<&Path> = step.and_then(|x| x.path.as_ref());

  // This is awful, what is the idiomatic way to "Chain" options?
  let mac: Option<&PathPlatform> = path.and_then(|x| x.mac.as_ref());
  let arch: Option<&PathPlatform> = path.and_then(|x| x.arch.as_ref());
  let windows: Option<&PathPlatform> = path.and_then(|x| x.windows.as_ref());
  let platform = if utils::is_mac() {
    mac
  } else if utils::is_arch_linux() {
    arch
  } else if utils::is_windows() {
    windows
  } else {
    None
  };

  let empty = vec![];

  let dirs: Option<&Vec<Value>> = path.and_then(|x| x.dirs.as_ref());
  let dirs = dirs.unwrap_or(&empty);

  let pdirs: Option<&Vec<Value>> = platform.and_then(|x| x.dirs.as_ref());
  let pdirs = pdirs.unwrap_or(&empty);

  let mut result: Vec<String> = Vec::new();
  for dir in dirs {
    if dir.is_str() {
      let s = String::from(dir.as_str().unwrap());
      result.push(s);
    }
  }

  for dir in pdirs {
    if dir.is_str() {
      let s = String::from(dir.as_str().unwrap());
      result.push(s);
    }
  }
  return result;
}

pub fn run(config: &Config) {
  let expected = get_path_dirs(config);

  let mut missing: Vec<&String> = vec![];

  let path = std::env::var("PATH");
  let home = dirs::home_dir()
    .map(|x| x.to_str().map(|x| x.to_string()).unwrap_or("~".to_string()))
    .unwrap_or("~".to_string());

  match path {
    | Err(_) => out(2, "Error getting $PATH, maybe it's not set?".red()),
    | Ok(path) => {
      let parts: Vec<String> =
        path.split(":").into_iter().map(|x| String::from(x)).collect();

      for item in expected.iter() {
        let mut exists = false;
        for part in parts.iter() {
          if item.starts_with("~") {
            // TODO: Join paths properly using PathBuf or something.
            let con = format!("{}{}", home, &item[1..]);
            if part == &con {
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
    "Found {}/{} expected locations",
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
