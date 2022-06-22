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

// Boilerplate for all the platforms.

trait Platformed<T> {
  fn arch(&self) -> &Option<T>;
  fn mac(&self) -> &Option<T>;
  fn windows(&self) -> &Option<T>;
  fn get_platformed(&self) -> &Option<T> {
    let res = if platform::is_mac() {
      self.mac()
    } else if platform::is_arch_linux() {
      self.arch()
    } else if platform::is_windows() {
      self.windows()
    } else {
      &None
    };
    res
  }
}

impl Platformed<PathBase> for Path {
  fn mac(&self) -> &Option<PathBase> {
    &self.mac
  }
  fn arch(&self) -> &Option<PathBase> {
    &self.arch
  }
  fn windows(&self) -> &Option<PathBase> {
    &self.windows
  }
}

// Implementations for each step's struct.

impl Path {
  fn dirs(&self) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    match &self.dirs {
      | None => (),
      | Some(dirs) => {
        for dir in dirs {
          if dir.is_str() {
            result.push(String::from(dir.as_str().unwrap()));
          }
        }
      },
    };

    match &self.get_platformed() {
      | None => (),
      | Some(base) => match &base.dirs {
        | None => (),
        | Some(dirs) => {
          for dir in dirs {
            if dir.is_str() {
              result.push(String::from(dir.as_str().unwrap()));
            }
          }
        },
      },
    };

    result
  }
}

// Overall Implementation.

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
    let empty = vec![];
    let dirs = match &self.step {
      | None => empty,
      | Some(step) => match &step.path {
        | None => empty,
        | Some(path) => path.dirs(),
      },
    };
    return dirs;
  }
}
