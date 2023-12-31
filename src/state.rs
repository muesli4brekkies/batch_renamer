use crate::{args, names, print};
use std::{fs, io, time};

pub struct State {
  pub is_verb: bool,
  pub is_quiet: bool,
  pub is_exec: bool,
  pub is_sort: bool,
}

impl State {
  pub fn get() -> State {
    State {
      is_verb: args::args_contain("v"),
      is_quiet: args::args_contain("q"),
      is_exec: args::args_contain("x"),
      is_sort: args::args_contain("s"),
    }
  }
}

pub fn run() -> io::Result<()> {
  args::check_args();
  run_loop(time::Instant::now(), names::get_names(), true)
}

pub fn run_loop(
  start_time: time::Instant,
  file_list: Vec<names::Names>,
  to_tmp: bool,
) -> io::Result<()> {
  let state = State::get();
  file_list.iter().try_for_each(|n| -> io::Result<()> {
    let (from, to) = match to_tmp {
      true => (&n.old, &n.tmp),
      false => (&n.tmp, &n.new),
    };
    if state.is_verb && !state.is_quiet {
      println!("{from} >> {to}")
    };
    if state.is_exec {
      fs::rename(from, to)?
    }
    Ok(())
  })?;
  match to_tmp {
    true => run_loop(start_time, file_list, false),
    false => {
      if !state.is_quiet {
        print::info(start_time, file_list.len() as f32, state)
      }
      Ok(())
    }
  }
}
