mod config;
mod out;
mod steps;
mod utils;

use std::path::PathBuf;

use colored::Colorize;
use config::parse::{self, Config};
use out::out;

fn _check_common_commands() {
  let commands = [
    "sudo", "exa", "htop", "git", "hg", "cargo", "rustup", "fd", "ripgrep",
    "vim", "nvim", "nano", "tmux", "nvm",
  ];

  // Should use get_max_len, see path.rs.
  let max_len = 20;

  for command in commands {
    let exists = utils::has_command(command);
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

pub fn run(path: PathBuf) {
  let config = parse::parse(path);

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
}
