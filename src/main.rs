use glob::glob;
use std::thread::ScopedJoinHandle;
use std::{env, fs, io, thread};
use walkdir::WalkDir;

fn rename_to_new_name(
  num_files: usize,
  dir: &String,
  is_go: bool,
  is_quiet: bool,
) -> Result<(), io::Error> {
  for i in 0..num_files {
    let dir_vec: Vec<&str> = dir.split('/').collect();
    if dir_vec.len() > 1 {
      let file_name = dir_vec[(dir_vec.len() - 2)..].join("");
      if !is_quiet {
        println!("{}/{}.tmp >r> {}/{}{}.jpg", dir, i, dir, file_name, i);
      }
      if is_go {
        fs::rename(
          format!("{}/{}.tmp", dir, i),
          format!("{}/{}{}.jpg", dir, file_name, i),
        )?;
      }
    }
  }
  Ok(())
}

fn rename_to_tmp(
  valid_files: Vec<String>,
  dir: &String,
  is_go: bool,
  is_quiet: bool,
) -> Result<(), io::Error> {
  for (i, file) in valid_files.iter().enumerate() {
    if !is_quiet {
      println!("{} >t> {}/{}.tmp", file, dir, i);
    }
    if is_go {
      fs::rename(file, format!("{}/{}.tmp", dir, i))?;
    }
  }
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
  let mut is_quiet = false;
  let mut is_help_or_default = false;
  let mut glob = String::from("*.jpg");
  for arg in &args {
    match arg.as_str() {
      "-x" => is_go = true,
      "-q" => is_quiet = true,
      "-h" => is_help_or_default = true,
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

  (is_go, is_quiet, is_help_or_default, glob)
}

fn print_help_and_gtfo() {
  println!(r#"batch_renamer - Renames files after previous directories");
          ----
          -usage - batch_renamer <args> <\"glob-string\">
          ----
          -options -x               - Execute renaming. Use with caution.
          -q               - Suppress terminal printing.
          -g \"glob_string\" - Optional. This prog defaults to globbing \"*.jpg\" files, but any similar glob can be searched for.
          -h               - Print this screen and exit."#);
  std::process::exit(0)
}

fn main() -> io::Result<()> {
  let (is_go, is_quiet, is_help_or_default, glob) = handle_args();
  if is_help_or_default {
    print_help_and_gtfo();
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
        let num_files: usize = valid_files.len();
        rename_to_tmp(valid_files, &dir, is_go, is_quiet)?;
        rename_to_new_name(num_files, &dir, is_go, is_quiet)?;
        Ok(())
      }));
      for child in children {
        child.join().unwrap()?;
      }
      Ok::<(), io::Error>(())
    })?;
  }
  if !is_go {
    println!("This was a dry-run. Pass \"-x\" as an argument to execute renaming");
    print_help_and_gtfo();
  } else {
    println!("Renaming executed")
  };
  Ok(())
}
