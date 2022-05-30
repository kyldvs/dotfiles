use std::path::PathBuf;

pub fn _get_home() -> String {
  let home = dirs::home_dir();
  return home
    .unwrap_or(PathBuf::from("~"))
    .to_str()
    .unwrap_or("~")
    .to_string();
}
