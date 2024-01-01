use crate::print;
use std::env;
pub fn check_args() {
  match (args_contain("x"), args_contain("p"), args_contain("h")) {
    (.., true) => print::ERRORS.help.print(),
    (true, true, _) => print::ERRORS.arg_clash.print(),
    (false, false, _) => print::ERRORS.no_run.print(),
    _ => {}
  }
}

pub fn args_contain(c: &str) -> bool {
  env::args().any(|arg| arg.starts_with('-') && arg.contains(c))
}

pub fn get_glob_arg() -> String {
  Arg::get_arg(Arg::Glob)
}

pub fn get_dir_arg() -> String {
  Arg::get_arg(Arg::Dir)
}

enum Arg {
  Glob,
  Dir,
}

impl Arg {
  fn get_arg(self) -> String {
    let default = String::from(if let Arg::Glob = self { "*.jpg" } else { "." });
    let filter_next = |a, (i, arg)| match env::args().nth(i + 1) {
      Some(r) if arg == if let Arg::Glob = self { "-g" } else { "-d" } => Some(r),
      _ => a,
    };

    env::args()
      .enumerate()
      .fold(None, filter_next)
      .unwrap_or(default)
  }
}
