use std::path::{Path, PathBuf};

pub fn _get_home() -> String {
  let home = dirs::home_dir();
  return home
    .unwrap_or(PathBuf::from("~"))
    .to_str()
    .unwrap_or("~")
    .to_string();
}

pub fn all_files_recursive<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
  // TODO: Error handling.
  let expanded = expand_tilde(path).unwrap();
  let glob_string = format!("{}/**/*", expanded.to_str().unwrap());
  let glob_str = glob_string.as_str();
  let mut v = vec![];
  for entry in glob::glob(glob_str).unwrap() {
    v.push(entry.unwrap());
  }
  v
}

pub fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
  let p = path_user_input.as_ref();
  if !p.starts_with("~") {
    return Some(p.to_path_buf());
  }
  if p == Path::new("~") {
    return dirs::home_dir();
  }
  dirs::home_dir().map(|mut h| {
    if h == Path::new("/") {
      // Corner case: `h` root directory;
      // don't prepend extra `/`, just drop the tilde.
      p.strip_prefix("~").unwrap().to_path_buf()
    } else {
      h.push(p.strip_prefix("~/").unwrap());
      h
    }
  })
}

#[test]
fn test_expand_tilde() {
  let home = std::env::var("HOME").unwrap();
  let projects = PathBuf::from(format!("{}/Projects", home));
  assert_eq!(expand_tilde("~/Projects"), Some(projects));
  assert_eq!(expand_tilde("/foo/bar"), Some("/foo/bar".into()));
  assert_eq!(expand_tilde("~alice/projects"), Some("~alice/projects".into()));
}
