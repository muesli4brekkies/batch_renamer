use glob::glob;
use std::thread::JoinHandle;
use std::{env, fs, io, thread};
use walkdir::WalkDir;

fn rename_to_new_name(num_files: usize, dir: &String, is_go: bool) -> Result<(),io::Error> {
  for i in 0..num_files {
    let dir_vec: Vec<&str> = dir.split('/').collect();
    if dir_vec.len() > 1 {
      let file_name = &dir_vec[(dir_vec.len() - 2)..].join("");
      println!("{}/{}.tmp >r> {}/{}{}.jpg", dir, i, dir, file_name, i);
      if is_go {
        fs::rename(
          format!("{}/{}.tmp", dir, i),
          format!("{}/{}{}.jpg", dir, file_name, i),
        )
        ?;
      }
    }
  }
  Ok(())
}

fn rename_to_tmp(valid_files: Vec<String>, dir: &String, is_go: bool) -> Result<(),io::Error> {
  for (i, file) in valid_files.iter().enumerate() {
    println!("{} >t> {}/{}.tmp", file, dir, i);
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

fn glob_files(dir: &String) -> Result<Vec<String>,glob::PatternError> {
  Ok(glob(&format!("{}/*.jpg", dir))?
    .map(|e| e.unwrap().into_os_string().into_string().unwrap())
    .collect())
}

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let is_go: bool = args.len() > 1 && args[1] == "GO";
  let num_threads = thread::available_parallelism()?.get();
  let unique_dirs = get_directories();
  let mut children:Vec<JoinHandle<()>> = vec![];
  for dir in unique_dirs {
    if children.len() >= num_threads {
        for child in children {
            child.join().expect("poop")
        }
      children = vec![];
    }
    children.push(thread::spawn(move || {
      let valid_files = glob_files(&dir).expect("Glob pattern error!");
      let num_files = valid_files.len();
      rename_to_tmp(valid_files, &dir, is_go).expect("Temp file write error!");
      rename_to_new_name(num_files, &dir, is_go).expect("Rename from temp file error!");
    }));
  }
  children
    .into_iter()
    .for_each(|c: JoinHandle<()>| c.join().unwrap());
  if !is_go {
    println!("Pass \"GO\" as an argument to execute renaming")
  };
  Ok(())
}
