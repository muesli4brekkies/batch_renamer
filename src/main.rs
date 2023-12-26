use exif::{Field, In, Reader, Tag};
use glob::glob;
use itertools::Itertools;
use std::{
  env::args,
  fs::{rename, File},
  io::BufReader,
  time::SystemTime,
};
use walkdir::WalkDir;
const TEMP_NAME: &str = ".brtmp";

fn main() {
  let start_time = SystemTime::now();
  let (is_execute, is_verbose, is_sort, glob) = handle_args();
  if !is_verbose {
    println!("Terminal printing disabled, -v to enable.")
  }

  let file_list = get_file_list(is_sort, &glob);
  let tot_files = file_list.len() as f32;
  rename_files(file_list.clone(), is_verbose, is_execute, true);
  rename_files(file_list, is_verbose, is_execute, false);

  let time_elapsed = SystemTime::now()
    .duration_since(start_time)
    .unwrap()
    .as_secs_f32();
  println!(
    "{} files in {} seconds. {:.0} files/sec\n{}{}\nglob = \"{}\"",
    tot_files,
    time_elapsed,
    tot_files / time_elapsed,
    match is_execute {
      true => "Renaming executed.",
      false => "This was a practice run. -x to execute renaming. Be careful.",
    },
    match is_sort {
      true => "\nSorted by EXIF date.",
      false => "",
    },
    glob
  )
}

fn rename_files(
  file_list: Vec<(String, String, String)>,
  is_verbose: bool,
  is_execute: bool,
  to_temp: bool,
) {
  file_list.iter().for_each(|(old, temp, new)| {
    let from = if to_temp { old } else { temp };
    let to = if to_temp { temp } else { new };
    if is_verbose {
      println!("{from} >> {to}")
    };
    if is_execute {
      rename(from, to).expect("fail :(")
    };
  });
}

fn get_file_list(is_sort: bool, glob: &String) -> Vec<(String, String, String)> {
  get_directories()
    .iter()
    .flat_map(|dir| {
      get_files(dir, glob, is_sort)
        .enumerate()
        .map(|(i, file)| {
          (
            file.clone(),
            format!("{}/{}{}", &dir, i, TEMP_NAME),
            format!(
              "{}/{}{}.{}",
              dir,
              dir
                .rsplit('/')
                .enumerate()
                .fold("".to_string(), |a, (i, w)| if i < 2 {
                  [w, &a].join("")
                } else {
                  a
                }),
              i,
              file.split('.').last().unwrap_or("")
            ),
          )
        })
        .collect_vec()
    })
    .collect_vec()
}

fn get_directories() -> Vec<String> {
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

fn get_files(dir: &String, glob_str: &String, is_sort: bool) -> std::vec::IntoIter<String> {
  let files = glob(&[dir, glob_str].iter().join("/"))
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

fn handle_args() -> (bool, bool, bool, String) {
  let args_contain = |c| -> bool { args().any(|arg| arg.starts_with('-') && arg.contains(c)) };
  let glob = args()
    .enumerate()
    .fold(None, |a, (i, f)| match f == "-g" {
      true => match args().nth(i + 1) {
        Some(r) => Some(r.to_string()),
        None => a,
      },
      false => a,
    })
    .unwrap_or(String::from("*.jpg"));

  let (is_help, is_execute, is_verbose, is_practice, is_sort) = (
    args_contain("h"),
    args_contain("x"),
    args_contain("v"),
    args_contain("p"),
    args_contain("s"),
  );
  if is_help {
    print_help_and_gtfo()
  };
  if is_execute && is_practice {
    println!("\nArguments error: Don't mix -x and -p ya dingus!");
    print_help_and_gtfo()
  } else if !is_execute && !is_practice {
    println!(
      "\nArguments error: Specify -p for practice run (recommended) or -x to execute renaming."
    );
    print_help_and_gtfo()
  }
  (is_execute, is_verbose, is_sort, glob)
}

fn print_help_and_gtfo() {
  println!(
    r#"
usage - ./batch_renamer -[hvpxs] -g "glob-string"
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
