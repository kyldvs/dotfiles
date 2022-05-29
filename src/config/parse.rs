#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml::value::Array;

#[derive(Deserialize)]
pub struct Config {
  pub control: Option<Control>,
}

#[derive(Deserialize)]
pub struct Control {
  pub steps: Option<Array>,
  pub mac: Option<ControlMac>,
}

#[derive(Deserialize)]
pub struct ControlMac {
  pub disable_steps: Option<Array>,
}

pub fn parse(path: PathBuf) -> Config {
  let file_contents = fs::read_to_string(path).unwrap();
  let config: Config = toml::from_str(&file_contents).unwrap();
  return config;
}
