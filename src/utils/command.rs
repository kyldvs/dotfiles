use which::which;

pub fn has(command: &str) -> bool {
  let result = match which(command) {
    | Ok(_) => true,
    | Err(_) => false,
  };
  return result;
}
