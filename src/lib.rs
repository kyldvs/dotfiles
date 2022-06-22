mod config;
mod emoji;
mod step;
mod terminal;
mod utils;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use config::Config;
use terminal::Terminal;
use termion::color;

fn _check_common_commands() {
  let commands = [
    "sudo", "exa", "htop", "git", "hg", "cargo", "rustup", "fd", "ripgrep",
    "vim", "nvim", "nano", "tmux", "nvm",
  ];
  let _ = commands;
}

pub fn run(path: PathBuf) {
  // tui::activate(path);

  let mut t = Terminal::new();
  let frame = Duration::from_millis(16);

  t.append(format!(
    "{}Parsing config:{}",
    color::Fg(color::Blue),
    color::Fg(color::Reset)
  ));

  t.append(format!("  {}", path.display()));

  t.append(format!(
    "{}Running structured steps:{}",
    color::Fg(color::Blue),
    color::Fg(color::Reset)
  ));

  let config = Config::from_path(path);
  let steps = step::get_steps();
  for step in steps {
    thread::sleep(frame);
    step.run(&mut t, &config);
  }
}
