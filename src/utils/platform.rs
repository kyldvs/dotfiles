use super::command;

pub fn is_windows() -> bool {
  return std::env::consts::OS == "windows";
}

pub fn is_mac() -> bool {
  return std::env::consts::OS == "macos";
}

pub fn is_arch_linux() -> bool {
  return std::env::consts::OS == "linux" && command::has("pacman");
}
