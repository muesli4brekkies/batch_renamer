use exif::{Field, In, Reader, Tag};
use glob::glob as globfiles;
use itertools::Itertools;
use std::{
  env::args,
  fs::{rename, File},
  io::BufReader,
  time::SystemTime,
};
use walkdir::WalkDir;

struct Bools {
  is_verb: bool,
  is_exec: bool,
  is_sort: bool,
}
pub fn run() {
  rename_files(get_start_time(), get_file_list(), true);
}

fn get_start_time() -> SystemTime {
  SystemTime::now()
}

fn rename_files(start_time: SystemTime, file_list: Vec<(String, String, String)>, to_tmp: bool) {
  let bools = get_args();
  file_list.iter().for_each(|(old, tmp, new)| {
    let (from, to) = if to_tmp { (old, tmp) } else { (tmp, new) };
    if bools.is_verb {
      println!("{from} >> {to}")
    };
    if bools.is_exec {
      rename(from, to).unwrap()
    };
  });
  if to_tmp {
    rename_files(start_time, file_list, false)
  } else {
    print_info(bools, start_time, file_list.len() as f32)
  }
}

fn get_file_list() -> Vec<(String, String, String)> {
  get_dirs()
    .iter()
    .flat_map(|dir| {
      get_files(dir, get_args().is_sort)
        .enumerate()
        .map(move |(i, file)| {
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
        })
    })
    .collect_vec()
}

fn print_info(bools: Bools, start_time: SystemTime, num_files: f32) {
  let time_elapsed = SystemTime::now()
    .duration_since(start_time)
    .unwrap()
    .as_secs_f32();
  println!(
    "{} files in {} seconds. {:.0} files/sec\n{}\n{}\nglob = \"{}\"",
    num_files,
    time_elapsed,
    num_files / time_elapsed,
    match bools.is_exec {
      true => "Renaming executed.",
      false => "This was a practice run. -x to execute renaming. Be careful.",
    },
    match bools.is_sort {
      true => "Sorted by EXIF date.",
      false => "NOT sorted",
    },
    get_glob()
  )
}

fn get_dirs() -> Vec<String> {
  WalkDir::new(".")
    .min_depth(2)
    .into_iter()
    .filter_map(|string| match string {
      Ok(s) => s
        .file_type()
        .is_dir()
        .then(|| s.into_path().to_string_lossy().to_string()),
      Err(_) => None,
    })
    .collect_vec()
}

fn get_files(dir: &String, is_sort: bool) -> std::vec::IntoIter<String> {
  let files = globfiles(&[dir, &get_glob()].iter().join("/"))
    .expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
    .map(|path_buff| path_buff.unwrap().into_os_string().into_string().unwrap());
  if is_sort {
    files.sorted_by_key(|f| {
      match Reader::new().read_from_container(&mut BufReader::new(File::open(f).unwrap())) {
        Ok(exif) => exif
          .get_field(Tag::DateTime, In::PRIMARY)
          .map(|date| Field::display_value(date).to_string()),
        Err(_) => None,
      }
      .unwrap_or(String::from('0'))
    })
  } else {
    files.sorted()
  }
}

fn get_glob() -> String {
  args()
    .enumerate()
    .fold(None, |a, (i, arg)| match arg == "-g" {
      true => match args().nth(i + 1) {
        Some(r) => Some(r),
        None => a,
      },
      false => a,
    })
    .unwrap_or(String::from("*.jpg"))
}

fn get_args() -> Bools {
  let args_contain = |c| -> bool { args().any(|arg| arg.starts_with('-') && arg.contains(c)) };
  let bools: Bools = Bools {
    is_verb: args_contain("v"),
    is_exec: args_contain("x"),
    is_sort: args_contain("s"),
  };
  let is_practice = args_contain("p");
  if args_contain("h") {
    print_help("")
  };
  match (bools.is_exec, is_practice) {
    (true, true) => print_help("\nArguments ERROR: Don't mix -x and -p ya dingus!\n\n"),
    (false, false) => print_help(
      "\nArguments ERROR: Specify -p for practice run (recommended) or -x to execute renaming.\n\n",
    ),
    _ => {}
  }
  bools
}

fn print_help(err: &str) {
  println!(
    r#"{err}usage - ./batch_renamer -[hvpxs] -g "glob-string"
 e.g. - ./batch_renamer -xvs -g "*.png"
                  ----
options 
        -h               - Print this screen and exit.
        -v               - Verbose terminal printing.
        -p               - Practice run. Combine with -v to print what the script will do.
        -x               - Execute renaming. Use with caution.
        -s               - Optional Sort by EXIF timestamp ascending. Defaults to simple alphanumeric filename sort.
        -g "glob_string" - Optional string to glob files with. Defaults to "*.jpg".
        "#
  );
  std::process::exit(0)
}
