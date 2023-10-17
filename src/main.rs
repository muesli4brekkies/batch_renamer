use glob::glob;
use std::thread::ScopedJoinHandle;
use std::{env, fs, io, thread};
use walkdir::WalkDir;
const TEMP_NAME: &'static str = ".batcher_renamertmp";

fn rename_to_new_name(
  num_files: usize,
  dir: &String,
  file_ext_list: Vec<&str>,
  is_go: bool,
  is_verbose: bool,
) -> Result<(), io::Error> {
  for i in 0..num_files {
    let dir_vec: Vec<&str> = dir.split('/').collect();
    let file_name = dir_vec[(dir_vec.len() - 2)..].join("");
    if is_verbose {
      println!(
        "{}/{}{} >r> {}/{}{}.{}",
        dir, i, dir, file_name, i, file_ext_list[i], TEMP_NAME
      );
    }
    if is_go {
      fs::rename(
        format!("{}/{}{}", dir, i, TEMP_NAME),
        format!("{}/{}{}.{}", dir, file_name, i, file_ext_list[i]),
      )?;
    }
  }
  Ok(())
}

fn rename_files(
  valid_files: Vec<String>,
  dir: &String,
  is_go: bool,
  is_verbose: bool,
) -> Result<(), io::Error> {
  let mut file_ext_list: Vec<&str> = vec![];
  for (i, file) in valid_files.iter().enumerate() {
    match file.split('.').last() {
      Some(file_ext) => file_ext_list.push(file_ext),
      None => file_ext_list.push(""),
    }

    if is_verbose {
      println!("{} >t> {}/{}{}", file, dir, i, TEMP_NAME);
    }
    if is_go {
      fs::rename(file, format!("{}/{}{}", dir, i, TEMP_NAME))?;
    }
  }
  rename_to_new_name(valid_files.len(), &dir, file_ext_list, is_go, is_verbose)?;
  Ok(())
}

fn get_directories() -> Vec<String> {
  WalkDir::new(".")
    .min_depth(2)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| e.file_type().is_dir())
    .map(|e| e.into_path().to_string_lossy().to_string())
    .collect()
}

fn glob_files(dir: &String, glob_str: &String) -> Result<Vec<String>, glob::PatternError> {
  Ok(
    glob(&format!("{}/{}", dir, glob_str))?
      .map(|e| e.unwrap().into_os_string().into_string().unwrap())
      .collect(),
  )
}

fn handle_args() -> (bool, bool, bool, String) {
  let args: Vec<String> = env::args().collect();
  let mut is_go = false;
  let mut is_verbose = false;
  let mut is_practice_run = false;
  let mut glob = String::from("*.jpg");
  for arg in &args {
    if args.len() == 1 {
      print_help_and_gtfo()
    };
    match arg.as_str() {
      "-v" => is_verbose = true,
      "-x" => is_go = true,
      "-xv" => {
        is_go = true;
        is_verbose = true
      }
      "-vx" => {
        is_go = true;
        is_verbose = true
      }
      "-p" => is_practice_run = true,
      "-pv" => {
        is_practice_run = true;
        is_verbose = true
      }
      "-vd" => {
        is_practice_run = true;
        is_verbose = true
      }
      "-h" => print_help_and_gtfo(),
      "-g" => {
        let mut i = 0;
        for arg in &args {
          i += 1;
          if args.len() >= i && arg == "-g" {
            glob = args[i].to_string();
          }
        }
      }
      _ => {}
    }
  }

  (is_go, is_verbose, is_practice_run, glob)
}

fn print_help_and_gtfo() {
  println!(
    r#"batch_renamer - Renames files after previous directories"
                  ----
usage - ./batch_renamer <args> <"glob-string">

for example:- ./batch_renamer -xv -g "*.png"
                  ----
options 
        -x               - Execute renaming. Use with caution.
        -v               - Enable terminal printing.
        -p               - Practice run. Combine with -v to print what the script will do.
        -g "glob_string" - Optional. This prog defaults to globbing \"*.jpg\" files, but any similar glob can be searched for.
        -h               - Print this screen and exit."#
  );
  std::process::exit(0)
}

fn main() -> io::Result<()> {
  let (is_go, is_verbose, is_practice_run, glob) = handle_args();
  if is_practice_run || is_go && !is_verbose {
    println!("Terminal printing disabled, -v to enable.")
  }
  let num_threads: usize = thread::available_parallelism()?.get();
  let unique_dirs: Vec<String> = get_directories();
  for dir in unique_dirs {
    thread::scope(|s| {
      let mut children: Vec<ScopedJoinHandle<Result<(), io::Error>>> = vec![];
      if children.len() >= num_threads {
        for child in children {
          child.join().unwrap()?;
        }
        children = vec![];
      }
      children.push(s.spawn(|| -> Result<(), io::Error> {
        let valid_files: Vec<String> = glob_files(&dir, &glob).expect("Glob pattern error!");
        rename_files(valid_files, &dir, is_go, is_verbose)?;
        Ok(())
      }));
      for child in children {
        child.join().unwrap()?;
      }
      Ok::<(), io::Error>(())
    })?;
  }
  if is_practice_run {
    println!("This was a practice run. -x to execute renaming. Be careful.");
  } else {
    println!("Renaming executed.")
  };
  Ok(())
}
