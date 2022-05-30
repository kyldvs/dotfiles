use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Helper to write and flush to stdout.
fn write<S: ToString>(s: S) {
  let mut stdout = stdout();
  write!(stdout, "{}", s.to_string()).unwrap();
  stdout.flush().unwrap();
}

fn writeln<S: ToString>(s: S) {
  let mut stdout = stdout();
  write!(stdout, "{}\r\n", s.to_string()).unwrap();
  stdout.flush().unwrap();
}

// Clear the entire terminal.
fn _clear() {
  write(termion::clear::All);
}

// Move the cursor to coordinates, (0, 0) is top left.
fn _move_cursor(x: u16, y: u16) {
  write(termion::cursor::Goto(x + 1, y + 1));
}

// Take control over the terminal.
pub fn activate(config_path: PathBuf) {
  let stdin = stdin();
  let _stdout = stdout().into_raw_mode().unwrap();

  let x = config_path.clone();
  let config_path_str = x.display();

  let _config = fs::read_to_string(config_path);

  writeln(format!(
    "{}Reading config:{}",
    color::Fg(color::Blue),
    color::Fg(color::Reset)
  ));

  writeln(format!(
    "  {}[x]{} {} {}...{} {}done{}",
    color::Fg(color::Green),
    color::Fg(color::Reset),
    config_path_str,
    color::Fg(color::LightBlack),
    color::Fg(color::Reset),
    color::Fg(color::Green),
    color::Fg(color::Reset),
  ));

  writeln(format!(
    "{}Running steps:{}",
    color::Fg(color::Blue),
    color::Fg(color::Reset)
  ));

  writeln(format!(
    "  {}[x]{} welcome {}...{} {}done{}",
    color::Fg(color::Green),
    color::Fg(color::Reset),
    color::Fg(color::LightBlack),
    color::Fg(color::Reset),
    color::Fg(color::Green),
    color::Fg(color::Reset),
  ));

  writeln(format!(
    "  {}[x]{} path {}......{} {}done{}",
    color::Fg(color::Green),
    color::Fg(color::Reset),
    color::Fg(color::LightBlack),
    color::Fg(color::Reset),
    color::Fg(color::Green),
    color::Fg(color::Reset),
  ));

  writeln(format!(
    "  {}[x]{} fonts {}.....{} {}done{}",
    color::Fg(color::Green),
    color::Fg(color::Reset),
    color::Fg(color::LightBlack),
    color::Fg(color::Reset),
    color::Fg(color::Green),
    color::Fg(color::Reset),
  ));

  for c in stdin.keys() {
    match c.unwrap() {
      // Exit codes
      | Key::Char('q') => break,
      | Key::Ctrl('c') => break,
      | Key::Ctrl('d') => break,
      | Key::Esc => break,

      // Simple test
      | Key::Char('w') => {
        write("test ");
      },
      | _ => (),
    }
  }

  // clear();
  // move_cursor(0, 0);
}
