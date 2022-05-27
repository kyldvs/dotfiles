mod config;
mod utils;

fn check_unix_common() {
  match utils::has_command("sudo") {
    | true => println!("> Has `sudo`"),
    | false => println!("> Missing `sudo`"),
  }
}

pub fn check() {
  if utils::is_windows() {
    panic!("Windows is not supported yet.")
  }

  if utils::is_mac() || utils::is_arch_linux() {
    check_unix_common();
  }
}
