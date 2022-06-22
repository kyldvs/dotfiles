#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml::value::Array;

use crate::utils::platform;

#[derive(Deserialize)]
pub struct Config {
  control: Option<Control>,
  step: Option<Step>,
}

#[derive(Deserialize)]
struct Control {
  steps: Option<Array>,
  disable_steps: Option<Array>,

  arch: Option<ControlBase>,
  mac: Option<ControlBase>,
  windows: Option<ControlBase>,
}

#[derive(Deserialize)]
struct ControlBase {
  steps: Option<Array>,
  disable_steps: Option<Array>,
}

#[derive(Deserialize)]
struct Step {
  path: Option<Path>,
}

#[derive(Deserialize)]
struct Path {
  dirs: Option<Array>,

  arch: Option<PathBase>,
  mac: Option<PathBase>,
  windows: Option<PathBase>,
}

#[derive(Deserialize)]
struct PathBase {
  dirs: Option<Array>,
}

impl Config {
  pub fn from_string(s: &String) -> Config {
    // TODO: Error handline when parsing toml.
    let config: Config = toml::from_str(s).unwrap();
    config
  }

  pub fn from_path(path: PathBuf) -> Config {
    // TODO: Error handling when reading file.
    let file_contents = fs::read_to_string(path).unwrap();
    Config::from_string(&file_contents)
  }

  pub fn get_path_dirs(&self) -> Vec<String> {
    let config = &self;
    let step = config.step.as_ref();
    let path = step.and_then(|x| x.path.as_ref());

    // This is awful, what is the idiomatic way to "Chain" options?
    let mac = path.and_then(|x| x.mac.as_ref());
    let arch = path.and_then(|x| x.arch.as_ref());
    let windows = path.and_then(|x| x.windows.as_ref());
    let platform = if platform::is_mac() {
      mac
    } else if platform::is_arch_linux() {
      arch
    } else if platform::is_windows() {
      windows
    } else {
      None
    };

    let empty = vec![];

    let dirs = path.and_then(|x| x.dirs.as_ref());
    let dirs = dirs.unwrap_or(&empty);

    let pdirs = platform.and_then(|x| x.dirs.as_ref());
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
}
