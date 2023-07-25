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
  fonts: Option<Fonts>,
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

#[derive(Deserialize)]
struct Fonts {
  fonts: Option<Array>,
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
    let main = to_string_vec(&self.dirs);
    let platformed = match &self.get_platformed() {
      | None => vec![],
      | Some(base) => to_string_vec(&base.dirs),
    };
    [main, platformed].concat()
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

    match &self.step {
      | None => empty,
      | Some(step) => match &step.path {
        | None => empty,
        | Some(path) => path.dirs(),
      },
    }
  }

  pub fn get_fonts(&self) -> Vec<String> {
    let empty = vec![];

    match &self.step {
      | None => empty,
      | Some(step) => match &step.fonts {
        | None => empty,
        | Some(fonts) => to_string_vec(&fonts.fonts),
      },
    }
  }
}

// Some utils.

fn to_string_vec(value: &Option<Array>) -> Vec<String> {
  let empty = vec![];
  match &value {
    | None => empty,
    | Some(value) => {
      let mut result = vec![];
      for x in value {
        if x.is_str() {
          // Don't use ".to_string()", it will add extra double quotes.
          let s = x.as_str().unwrap();
          let s = String::from(s);
          result.push(s);
        }
      }
      result
    },
  }
}
