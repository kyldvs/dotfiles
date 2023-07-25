use which::which;

pub fn has(command: &str) -> bool {
  match which(command) {
    | Ok(_) => true,
    | Err(_) => false,
  }
}
