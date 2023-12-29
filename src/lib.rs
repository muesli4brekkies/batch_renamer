pub fn run() {
  state::recurse(std::time::SystemTime::now(), get::file_list(), true);
}

mod state {
  use crate::{get::Names, print};
  use std::env;
  use std::{fs::rename, time::SystemTime};

  pub fn recurse(start_time: SystemTime, file_list: Vec<Names>, to_tmp: bool) {
    check_args();
    let state = get_state();
    file_list.iter().for_each(|n| {
      (|(to, from): (&String, &String)| {
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
      true => recurse(start_time, file_list, false),
      false => {
        if !state.is_quiet {
          print::info(start_time, file_list.len() as f32)
        }
      }
    }
  }

  pub struct States {
    pub is_verb: bool,
    pub is_quiet: bool,
    pub is_exec: bool,
    pub is_sort: bool,
  }

  struct Errors<'a> {
    help: &'a str,
    arg_clash: &'a str,
    no_run: &'a str,
  }

  pub(crate) fn get_state() -> States {
    States {
      is_verb: args_contain("v"),
      is_quiet: args_contain("q"),
      is_exec: args_contain("x"),
      is_sort: args_contain("s"),
    }
  }

  fn check_args() {
    let errors = Errors {
      help: "\nHELP:\n\n",
      arg_clash: "\nERROR: Don't mix -x and -p ya dingus!\n\n",
      no_run: "\nERROR: Need -x or -p to run\n\n",
    };
    match (
      args_contain("x") as usize,
      args_contain("p") as usize,
      args_contain("h") as usize,
    ) {
      (_, _, 1) => print::help(errors.help),
      (1, 1, _) => print::help(errors.arg_clash),
      (0, 0, _) => print::help(errors.no_run),
      _ => {}
    }
  }

  fn args_contain(c: &str) -> bool {
    env::args().any(|arg| arg.starts_with('-') && arg.contains(c))
  }
}

mod get {
  use crate::state::get_state;
  use exif::{Field, In, Reader, Tag};
  use glob::glob;
  use itertools::Itertools;
  use std::{env, fs::File, io::BufReader};
  use walkdir::{DirEntry, WalkDir};

  pub fn file_list() -> Vec<Names> {
    let is_sort = get_state().is_sort;

    WalkDir::new(glob_or_dir(false))
      .min_depth(2)
      .into_iter()
      .filter_map(|dir| match dir {
        Ok(r) => get_dir_jobs(r, is_sort),
        Err(_) => None,
      })
      .flatten()
      .collect_vec()
  }

  pub fn glob_or_dir(is_glob: bool) -> String {
    env::args()
      .enumerate()
      .fold(None, |a, (i, arg)| {
        match arg == if is_glob { "-g" } else { "-d" } {
          true => match env::args().nth(i + 1) {
            Some(r) => Some(r),
            None => a,
          },
          false => a,
        }
      })
      .unwrap_or(String::from(if is_glob { "*.jpg" } else { "." }))
  }
  pub struct Names {
    pub old: String,
    pub tmp: String,
    pub new: String,
  }

  fn names_tuple(i: usize, file: String) -> Names {
    let dir = file.rsplit('/').dropping(1);
    let dir_str = dir.clone().rev().join("/");
    Names {
      tmp: format!("{}{}{}", dir_str, i, ".brtmp"),
      new: format!(
        "{}/{}{}.{}",
        dir_str,
        dir.take(2).fold(String::new(), |a, w| [w, &a].join("")),
        i,
        file.split('.').last().unwrap_or("")
      ),
      old: file,
    }
  }

  fn get_files(is_sort: bool, dir: DirEntry) -> std::vec::IntoIter<String> {
    let exif_date = |file: &String| -> String {
      match Reader::new().read_from_container(&mut BufReader::new(File::open(file).unwrap())) {
        Ok(exif) => exif
          .get_field(Tag::DateTime, In::PRIMARY)
          .map(|date| Field::display_value(date).to_string()),
        Err(_) => None,
      }
      .unwrap_or(String::from('0'))
    };
    let glob_pattern = &[
      dir.into_path().as_os_str().to_string_lossy().to_string(),
      glob_or_dir(true),
    ]
    .join("/");
    let files = glob(glob_pattern)
      .expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
      .map(|path_buff| path_buff.unwrap().into_os_string().into_string().unwrap());
    if is_sort {
      files.sorted_by_key(exif_date)
    } else {
      files.sorted()
    }
  }

  fn get_dir_jobs(dir: DirEntry, is_sort: bool) -> Option<Vec<Names>> {
    match dir.file_type().is_dir() {
      true => Some(
        get_files(is_sort, dir)
          .enumerate()
          .map(|(i, file)| names_tuple(i, file))
          .collect_vec(),
      ),
      false => None,
    }
  }
}

mod print {
  use crate::{get::glob_or_dir, state::get_state};
  use std::time::SystemTime;

  pub fn info(start_time: SystemTime, num_files: f32) {
    let (glob, root_dir, state) = (glob_or_dir(true), glob_or_dir(false), get_state());
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
      glob,
      root_dir
    );
  }

  pub fn help(err: &str) {
    println!(
      r#"{err}usage - ./batch_renamer -h -[vq] -[px] -s -g "glob-string" -d <path>
 e.g. - ./batch_renamer -xvs -g "*.png" -d ./directory
                  ----
options 

        -v               - Verbose terminal printing.
        -q               - Disable terminal printing entirely. Overrides -v.

        -p               - Practice run. Combine with -v to print what the script will do!
        -x               - Execute renaming. Use with caution.

        -s               - Optional Sort by EXIF timestamp ascending. Defaults to simple alphanumeric filename sort.
        -g "glob_string" - Optional string to glob files with.        Defaults to "*.jpg".
        -d <path>        - Optional path to run search from.          Defaults to directory the binary is run from.
        "#
    );
    std::process::exit(2)
  }
}
