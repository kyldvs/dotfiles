pub mod command;
pub mod path;
pub mod platform;

use core::slice::Iter;
use std::cmp::max;

pub fn get_max_len(items: Iter<&String>) -> usize {
  let len = items.fold(0, |acc, c| max(acc, c.len()));
  return len;
}
