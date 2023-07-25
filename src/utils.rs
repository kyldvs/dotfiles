pub mod command;
pub mod path;
pub mod platform;

use core::slice::Iter;
use std::cmp::max;

pub fn _get_max_len(items: Iter<&str>) -> usize {
  items.fold(0, |acc, c| max(acc, c.len()))
}
