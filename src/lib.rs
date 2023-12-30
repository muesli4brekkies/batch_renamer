pub fn run() {
  state::run_loop(std::time::SystemTime::now(), get::dirs(), true);
}

mod state {
  use crate::{get::Names, print};
  use args::{args_contain, check_args};
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
        is_verb: args_contain("v"),
        is_quiet: args_contain("q"),
        is_exec: args_contain("x"),
        is_sort: args_contain("s"),
      }
    }
  }

  pub fn run_loop(start_time: SystemTime, file_list: Vec<Names>, to_tmp: bool) {
    check_args();
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

  pub mod args {
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
      Arg::arg_get(Arg::Glob)
    }

    pub fn get_dir_arg() -> String {
      Arg::arg_get(Arg::Dir)
    }

    enum Arg {
      Glob,
      Dir,
    }
    impl Arg {
      fn arg_get(self) -> String {
        let default = String::from(if let Arg::Glob = self { "*.jpg" } else { "." });
        env::args()
          .enumerate()
          .fold(None, |a, (i, arg)| {
            match arg == if let Arg::Glob = self { "-g" } else { "-d" } {
              true => match env::args().nth(i + 1) {
                Some(r) => Some(r),
                None => a,
              },
              false => a,
            }
          })
          .unwrap_or(default)
      }
    }
  }
}
mod get {
  use crate::state::{
    args::{get_dir_arg, get_glob_arg},
    State,
  };
  use exif::{Field, In, Reader, Tag};
  use itertools::Itertools;
  use std::{fs::File, io::BufReader, path::PathBuf};
  use walkdir::{DirEntry, WalkDir};

  pub struct Names {
    pub old: String,
    pub tmp: String,
    pub new: String,
  }

  impl Names {
    fn get((i, file): (usize, String)) -> Names {
      let dir = file.rsplit('/').dropping(1);
      let dir_str = dir.clone().rev().join("/");
      Names {
        new: format!(
          "{}/{}{}.{}",
          dir_str,
          dir.take(2).fold(String::new(), |a, w| [w, &a].join("")),
          i,
          file.split('.').last().unwrap_or("")
        ),
        tmp: format!("{}{}{}", dir_str, i, ".brtmp"),
        old: file,
      }
    }
  }

  pub fn dirs() -> Vec<Names> {
    let is_sort = State::get().is_sort;
    WalkDir::new(get_dir_arg())
      .min_depth(2)
      .into_iter()
      .filter_map(|dir| match dir {
        Ok(r) => r.file_type().is_dir().then(|| get_names(is_sort, r)),
        Err(_) => None,
      })
      .flatten()
      .collect_vec()
  }

  fn get_names(is_sort: bool, dir: DirEntry) -> Vec<Names> {
    use glob::{glob, GlobError};
    fn get_exif_date(file: &String) -> String {
      match Reader::new().read_from_container(&mut BufReader::new(File::open(file).unwrap())) {
        Ok(exif) => exif
          .get_field(Tag::DateTime, In::PRIMARY)
          .map(|date| Field::display_value(date).to_string()),
        Err(_) => None,
      }
      .unwrap_or(String::from('0'))
    }

    fn unwrap_pathbuf(path: Result<PathBuf, GlobError>) -> String {
      path.unwrap().into_os_string().into_string().unwrap()
    }

    fn dir_ent_to_string(dir: DirEntry) -> String {
      dir.into_path().as_os_str().to_string_lossy().to_string()
    }

    glob(&[dir_ent_to_string(dir), get_glob_arg()].join("/"))
      .expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
      .map(unwrap_pathbuf)
      .sorted_by_key(|f| match is_sort {
        true => get_exif_date(f),
        false => String::from("0"),
      })
      .enumerate()
      .map(Names::get)
      .collect_vec()
  }
}

mod print {
  use crate::state::{
    args::{get_dir_arg, get_glob_arg},
    State,
  };
  use std::time::SystemTime;

  pub struct Errors {
    pub help: Error,
    pub arg_clash: Error,
    pub no_run: Error,
  }

  pub const ERRORS: Errors = Errors {
    help: Error("\nHELP:\n\n"),
    arg_clash: Error("\nERROR: Don't mix -x and -p ya dingus!\n\n"),
    no_run: Error("\nERROR: Need -x or -p to run\n\n"),
  };

  pub struct Error(&'static str);

  impl Error {
    pub fn print(&self) {
      println!(
        r#"{}usage - ./batch_renamer -h -[vq] -[px] -s -g "glob-string" -d <path>
 e.g. - ./batch_renamer -xvs -g "*.png" -d ./directory
options 
        -v               - Verbose terminal printing.
        -q               - Disable terminal printing entirely. Overrides -v.

        -p               - Practice run. Combine with -v to print what the script will do!
        -x               - Execute renaming. Use with caution.

        -s               - Optional Sort by EXIF timestamp ascending. Defaults to simple alphanumeric filename sort.
        -g "glob_string" - Optional string to glob files with.        Defaults to "*.jpg".
        -d <path>        - Optional path to run search from.          Defaults to directory the binary is run from.
        "#,
        self.0
      );
      std::process::exit(2)
    }
  }

  pub fn info(start_time: SystemTime, num_files: f32, state: State) {
    let time_elapsed = SystemTime::now()
      .duration_since(start_time)
      .expect("\nTime has gone backwards. :(\n")
      .as_secs_f32();

    println!(
      "{} files in {} seconds. {:.0} files/sec\n{}\n{}\nglob = \"{}\"\nroot dir = \"{}\"",
      num_files,
      time_elapsed,
      num_files / time_elapsed,
      match state.is_exec {
        true => "Renaming executed.",
        false => "This was a practice run. -x to execute renaming. Be careful.",
      },
      match state.is_sort {
        true => "Sorted by EXIF date.",
        false => "NOT sorted",
      },
      get_glob_arg(),
      get_dir_arg()
    );
  }
}
