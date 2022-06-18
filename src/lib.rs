mod config;
mod emoji;
mod out;
mod step;
mod steps;
mod terminal;
mod utils;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use colored::Colorize;
use config::Config;
use out::out;
use step::{Path, Step};
use terminal::Terminal;
use termion::color;
use utils::command;

fn _check_common_commands() {
  let commands = [
    "sudo", "exa", "htop", "git", "hg", "cargo", "rustup", "fd", "ripgrep",
    "vim", "nvim", "nano", "tmux", "nvm",
  ];

  // Should use get_max_len, see path.rs.
  let max_len = 20;

  for command in commands {
    let exists = command::has(command);
    let result = if exists { "Good".green() } else { "Missing".red() };
    out(2, format!("{:max_len$}  {}", command, result));
  }
}

fn welcome(term: &mut Terminal) {
  term.append(format!("  [ ] welcome"));
}

fn check_path(term: &mut Terminal) {
  term.append(format!("  [ ] check_path"));
}

fn fonts(term: &mut Terminal) {
  term.append(format!("  [ ] fonts"));
}

fn settings(term: &mut Terminal) {
  term.append(format!("  [ ] settings"));
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
    "{}Running steps:{}",
    color::Fg(color::Blue),
    color::Fg(color::Reset)
  ));

  let steps = [welcome, check_path, fonts, settings];
  for step in steps {
    thread::sleep(frame);
    step(&mut t);
  }

  t.append(format!(
    "{}Running structured steps:{}",
    color::Fg(color::Blue),
    color::Fg(color::Reset)
  ));

  let config = config::parse(path);
  let steps = [Path::new()];
  for step in steps {
    thread::sleep(frame);
    step.run(&mut t, &config);
  }
}
