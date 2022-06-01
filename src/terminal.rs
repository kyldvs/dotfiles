use std::io::{stdout, Write};

use termion::color;

pub struct Line {
  string: String,
}

pub struct Terminal {
  width: usize,
  lines: Vec<Line>,
}

fn debug<S: ToString>(s: S) {
  if true {
    return;
  }

  println!(
    "{}[debug] {}{}",
    color::Fg(color::Blue),
    s.to_string(),
    color::Fg(color::Reset)
  );
}

fn warn<S: ToString>(s: S) {
  println!(
    "{}[warning] {}{}",
    color::Fg(color::Yellow),
    s.to_string(),
    color::Fg(color::Reset)
  );
}

fn flush() {
  let _ = stdout().flush();
}

fn to_u16(i: usize) -> u16 {
  if i > std::u16::MAX as usize {
    warn(format!("Too large to fit into u16: {}", i));
    std::u16::MAX
  } else {
    i as u16
  }
}

impl Terminal {
  pub fn new() -> Terminal {
    // Get the initial terminal size.
    // TODO: Recompute this in case it changes?
    let width = match termion::terminal_size() {
      | Ok((width, height)) => {
        debug(format!("Terminal size: ({}, {})", width, height));
        width
      },

      | Err(_) => {
        warn("Unable to get terminal size, defaulting to width 80.");
        80
      },
    } as usize;

    Terminal { width, lines: Vec::new() }
  }

  pub fn append<S: ToString>(&mut self, s: S) -> usize {
    let string = s.to_string();
    // We can simply print the string, we keep the cursor at the bottom left
    // so this will add the new line in the correct place.
    println!("{}", string);
    flush();
    self.lines.push(Line { string });
    self.lines.len() - 1
  }

  pub fn update<S: ToString>(&mut self, i: usize, s: S) {
    let n = self.lines.len();
    if i >= n {
      warn(format!(
        "Trying to update index {}, but there are only {} lines.",
        i, n,
      ));
      return;
    }

    // Don't need to update if it's the same.
    let string = s.to_string();
    if self.lines[i].string == string {
      return;
    }

    let up = to_u16(n - i);
    let left = to_u16(self.width);

    print!(
      "{}{}{}{}{}{}{}{}",
      // Move cursor to line being updated.
      termion::cursor::Up(up),
      termion::clear::CurrentLine,
      // Reset all styles.
      termion::color::Fg(termion::color::Reset),
      termion::color::Bg(termion::color::Reset),
      termion::style::Reset,
      // Print the new line.
      string,
      // Move cursor all the way down and to the left.
      termion::cursor::Left(left),
      termion::cursor::Down(up)
    );
    flush();

    self.lines[i] = Line { string };
  }
}
