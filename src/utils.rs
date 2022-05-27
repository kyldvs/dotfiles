use which::which;

pub fn has_command(command: &str) -> bool {
  let result = match which(command) {
    | Ok(_) => true,
    | Err(_) => false,
  };
  return result;
}

pub fn is_windows() -> bool {
  return std::env::consts::OS == "windows";
}

pub fn is_mac() -> bool {
  return std::env::consts::OS == "macos";
}

pub fn is_arch_linux() -> bool {
  return std::env::consts::OS == "linux" && has_command("pacman");
}
