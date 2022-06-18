#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml::value::Array;

#[derive(Deserialize)]
pub struct Config {
  pub control: Option<Control>,
  pub step: Option<Step>,
}

#[derive(Deserialize)]
pub struct Control {
  pub steps: Option<Array>,
  pub disable_steps: Option<Array>,

  pub arch: Option<ControlBase>,
  pub mac: Option<ControlBase>,
  pub windows: Option<ControlBase>,
}

#[derive(Deserialize)]
pub struct ControlBase {
  pub steps: Option<Array>,
  pub disable_steps: Option<Array>,
}

#[derive(Deserialize)]
pub struct Step {
  pub path: Option<Path>,
}

#[derive(Deserialize)]
pub struct Path {
  pub dirs: Option<Array>,

  pub arch: Option<PathBase>,
  pub mac: Option<PathBase>,
  pub windows: Option<PathBase>,
}

#[derive(Deserialize)]
pub struct PathBase {
  pub dirs: Option<Array>,
}

pub fn parse(path: PathBuf) -> Config {
  // TODO: Error handling when reading file.
  let file_contents = fs::read_to_string(path).unwrap();

  // TODO: Error handline when parsing toml.
  let config: Config = toml::from_str(&file_contents).unwrap();

  return config;
}
