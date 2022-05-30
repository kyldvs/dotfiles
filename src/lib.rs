mod out;
mod state;
mod steps;
mod tui;
mod utils;

use std::path::PathBuf;

use colored::Colorize;
use out::out;
use state::config::{self, Config};
use state::State;
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

// TODO: Move this into config or utils module at some point.
fn get_steps(config: &Config) -> Vec<String> {
  let empty = vec![];
  match &config.control {
    | None => empty,
    | Some(control) => match &control.steps {
      | None => empty,
      | Some(steps) => {
        let mut clean_steps = Vec::new();
        for step in steps.iter() {
          if step.is_str() {
            clean_steps.push(String::from(step.as_str().unwrap()));
          };
        }
        return clean_steps;
      },
    },
  }
}

pub fn _run(path: PathBuf) {
  let config = config::parse(path);

  let steps = get_steps(&config);

  // Add some padding at the start.
  out(0, "");

  for step_dynamic in steps {
    // This is probably terrible, strings in rust confuse me.
    let step = step_dynamic.as_str();

    out(1, format!("[step] {}", step).blue());
    match step {
      | "fonts" => steps::fonts::run(),
      | "path" => steps::path::run(&config),
      | "welcome" => steps::welcome::run(),
      | _ => out(2, "[warning] Unknown step".yellow()),
    }
    out(0, "");
  }

  // Say goodbye and add some padding at the end.
  out(1, "All done, goodbye!".green());
  out(0, "");

  // Try taking things over.
  let mut _s = State {
    // as
    raw_term: None,
  };
}

pub fn run(path: PathBuf) {
  tui::activate(path);
}
