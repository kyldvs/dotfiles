use std::io::Stdout;

use termion::raw::RawTerminal;

pub mod config;

pub struct State {
  pub raw_term: Option<RawTerminal<Stdout>>,
}
