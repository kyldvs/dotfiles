use super::{Path, Step};
use crate::config::Config;
use crate::terminal::Terminal;
use crate::utils::{self, platform};

impl Path {
  pub fn new() -> Path {
    Path
  }

  fn get_path_dirs(config: &Config) -> Vec<String> {
    let step = config.step.as_ref();
    let path = step.and_then(|x| x.path.as_ref());

    // This is awful, what is the idiomatic way to "Chain" options?
    let mac = path.and_then(|x| x.mac.as_ref());
    let arch = path.and_then(|x| x.arch.as_ref());
    let windows = path.and_then(|x| x.windows.as_ref());
    let platform = if platform::is_mac() {
      mac
    } else if platform::is_arch_linux() {
      arch
    } else if platform::is_windows() {
      windows
    } else {
      None
    };

    let empty = vec![];

    let dirs = path.and_then(|x| x.dirs.as_ref());
    let dirs = dirs.unwrap_or(&empty);

    let pdirs = platform.and_then(|x| x.dirs.as_ref());
    let pdirs = pdirs.unwrap_or(&empty);

    let mut result: Vec<String> = Vec::new();
    for dir in dirs {
      if dir.is_str() {
        let s = String::from(dir.as_str().unwrap());
        result.push(s);
      }
    }

    for dir in pdirs {
      if dir.is_str() {
        let s = String::from(dir.as_str().unwrap());
        result.push(s);
      }
    }
    return result;
  }
}

impl Step for Path {
  fn get_name(&self) -> String {
    String::from("path")
  }

  fn run(&self, t: &mut Terminal, c: &Config) {
    let name = self.get_name();
    let status_line = t.append(format!("  [ ] {}", name));

    let expected = Path::get_path_dirs(c);

    let mut missing: Vec<&String> = vec![];

    let path = std::env::var("PATH");
    let home = dirs::home_dir()
      .map(|x| x.to_str().map(|x| x.to_string()).unwrap_or("~".to_string()))
      .unwrap_or("~".to_string());

    match path {
      | Err(_) => {
        t.append("    > Error getting $PATH, maybe it's not set?");
      },
      | Ok(path) => {
        let parts: Vec<String> =
          path.split(":").into_iter().map(|x| String::from(x)).collect();

        for item in expected.iter() {
          let mut exists = false;
          for part in parts.iter() {
            if item.starts_with("~") {
              // TODO: Join paths properly using PathBuf or something.
              let con = format!("{}{}", home, &item[1..]);
              if part == &con {
                exists = true;
              }
            } else {
              if part == item {
                exists = true;
              }
            }

            if exists {
              break;
            }
          }

          // Put any missing items into the missing vector, used for reporting.
          if !exists {
            missing.push(item);
          }
        }
      },
    };

    let overview = format!(
      "Found {}/{} expected locations",
      expected.len() - missing.len(),
      expected.len()
    );

    let colored_overview = if missing.len() == 0 {
      format!(
        "{}{}{}",
        termion::color::Fg(termion::color::Green),
        overview,
        termion::color::Fg(termion::color::Reset)
      )
    } else {
      format!(
        "{}{}{}",
        termion::color::Fg(termion::color::Yellow),
        overview,
        termion::color::Fg(termion::color::Reset)
      )
    };

    t.append(format!("    {}", colored_overview));

    let max_len = utils::get_max_len(missing.iter());
    for item in missing {
      t.append(format!("    {:max_len$}  {}", item, "Missing"));
    }

    t.update(status_line, format!("  [x] {}", name))
  }
}
