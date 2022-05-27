fn get_indent_str(indent: usize) -> String {
  if indent <= 0 {
    return String::new();
  }
  return "  ".repeat(indent);
}

pub fn out<S: ToString>(indent: usize, text: S) {
  let indent_str = get_indent_str(indent);
  println!("{}{}", indent_str, text.to_string());
}
