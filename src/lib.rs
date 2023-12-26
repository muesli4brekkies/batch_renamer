pub mod run {
  use crate::{get, print, state};
  use std::{fs::rename, time::SystemTime};

  pub fn go() {
    rename_files(SystemTime::now(), get::file_list(), true);
  }

  fn rename_files(start_time: SystemTime, file_list: Vec<(String, String, String)>, to_tmp: bool) {
    let state = state::get_state();
    file_list.iter().for_each(|(old, tmp, new)| {
      let (from, to) = if to_tmp { (old, tmp) } else { (tmp, new) };
      if state.is_verb {
        println!("{from} >> {to}")
      };
      if state.is_exec {
        rename(from, to).unwrap()
      };
    });
    if to_tmp {
      rename_files(start_time, file_list, false)
    } else {
      print::info(start_time, file_list.len() as f32)
    }
  }
}

mod state {
  use crate::print;
  use std::env;

  pub(crate) struct State {
    pub(crate) is_verb: bool,
    pub(crate) is_exec: bool,
    pub(crate) is_sort: bool,
  }

  pub(crate) fn get_state() -> State {
    check_errors();
    State {
      is_verb: args_contain("v"),
      is_exec: args_contain("x"),
      is_sort: args_contain("s"),
    }
  }

  fn args_contain(c: &str) -> bool {
    env::args().any(|arg| arg.starts_with('-') && arg.contains(c))
  }

  fn check_errors() {
    match (args_contain("x"), args_contain("p"), args_contain("h")) {
      (_, _, true) => print::help("\nHELP:\n\n"),
      (true, true, _) => print::help("\nERROR: Don't mix -x and -p ya dingus!\n\n"),
      (false, false, _) => print::help("\nERROR: Need -x or -p to run\n\n"),
      _ => {}
    }
  }
}

mod get {
  use crate::state::get_state;
  use exif::{Field, In, Reader, Tag};
  use glob::glob;
  use itertools::Itertools;
  use std::{env, fs::File, io::BufReader};
  use walkdir::{DirEntry, WalkDir};

  pub(crate) fn glob_or_dir(is_glob: bool) -> String {
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

  pub(crate) fn file_list() -> Vec<(String, String, String)> {
    WalkDir::new(glob_or_dir(false))
      .min_depth(2)
      .into_iter()
      .filter_map(Result::ok)
      .flat_map(get_dir_jobs)
      .collect_vec()
  }

  fn get_dir_jobs(dir_entry: DirEntry) -> Vec<(String, String, String)> {
    fn jobs_tuple(i: usize, file: String, dir: String) -> (String, String, String) {
      (
        file.clone(),
        format!("{}/{}{}", dir, i, ".brtmp"),
        format!(
          "{}/{}{}.{}",
          dir,
          dir
            .rsplit('/')
            .take(2)
            .fold(String::from(""), |a, w| [w, &a].join("")),
          i,
          file.split('.').last().unwrap_or("")
        ),
      )
    }

    fn get_files(is_sort: bool, dir: &String) -> std::vec::IntoIter<String> {
      fn exif_date(file: &String) -> String {
        match Reader::new().read_from_container(&mut BufReader::new(File::open(file).unwrap())) {
          Ok(exif) => exif
            .get_field(Tag::DateTime, In::PRIMARY)
            .map(|date| Field::display_value(date).to_string()),
          Err(_) => None,
        }
        .unwrap_or(String::from('0'))
      }

      let files = glob(&[dir, &glob_or_dir(true)].iter().join("/"))
        .expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
        .map(|path_buff| path_buff.unwrap().into_os_string().into_string().unwrap());
      if is_sort {
        files.sorted_by_key(exif_date)
      } else {
        files.sorted()
      }
    }

    match dir_entry.file_type().is_dir() {
      true => {
        let dir = dir_entry
          .into_path()
          .as_os_str()
          .to_string_lossy()
          .to_string();
        get_files(get_state().is_sort, &dir)
          .enumerate()
          .map(move |(i, file)| jobs_tuple(i, file, dir.clone()))
          .collect_vec()
      }
      false => Vec::new(),
    }
  }
}

mod print {
  use crate::{get::glob_or_dir, state::get_state};
  use std::time::SystemTime;

  pub(crate) fn info(start_time: SystemTime, num_files: f32) {
    let (glob, dir, state) = (glob_or_dir(true), glob_or_dir(false), get_state());
    let time_elapsed = SystemTime::now()
      .duration_since(start_time)
      .unwrap()
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
      dir
    )
  }

  pub(crate) fn help(err: &str) {
    println!(
      r#"{err}usage - ./batch_renamer -[hvpxs] -g "glob-string" -d <path>
 e.g. - ./batch_renamer -xvs -g "*.png" -d ../
                  ----
options 
        -h               - Print this screen and exit.
        -v               - Verbose terminal printing.
        -p               - Practice run. Combine with -v to print what the script will do.
        -x               - Execute renaming. Use with caution.
        -s               - Optional Sort by EXIF timestamp ascending. Defaults to simple alphanumeric filename sort.
        -g "glob_string" - Optional string to glob files with.        Defaults to "*.jpg".
        -d <path>        - Optional path to run search from.          Defaults to directory the binary is run from.
        "#
    );
    std::process::exit(0)
  }
}
