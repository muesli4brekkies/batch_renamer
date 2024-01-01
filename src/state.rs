use crate::{args, names, print};
use std::{fs::rename, time::SystemTime};

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

pub fn run() {
  run_loop(std::time::SystemTime::now(), names::dirs(), true);
}

pub fn run_loop(start_time: SystemTime, file_list: Vec<names::Names>, to_tmp: bool) {
  args::check_args();
  let state = State::get();
  file_list.iter().for_each(|n| {
    (|(from, to): (&String, &String)| {
      if state.is_verb && !state.is_quiet {
        println!("{from} >> {to}")
      };
      if state.is_exec {
        rename(from, to).unwrap()
      };
    })(match to_tmp {
      true => (&n.old, &n.tmp),
      false => (&n.tmp, &n.new),
    })
  });
  match to_tmp {
    true => run_loop(start_time, file_list, false),
    false => {
      if !state.is_quiet {
        print::info(start_time, file_list.len() as f32, state)
      }
    }
  }
}
