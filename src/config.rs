// "Temporarily" allow dead code during development.
#![allow(dead_code)]

use serde::Deserialize;
use toml::value::Array;

#[derive(Deserialize)]
struct Config {
  common: Common,
  arch: Arch,
  mac: Mac,
}

#[derive(Deserialize)]
struct Common {
  required_commands: Array,
  commands: Array,
}

#[derive(Deserialize)]
struct Arch {
  package_manager: String,
}

#[derive(Deserialize)]
struct Mac {
  package_manager: String,
  programs: Array,
  settings: Array,
}
