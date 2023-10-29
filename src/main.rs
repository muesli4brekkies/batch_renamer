use exif;
use glob::glob;
use std::thread::ScopedJoinHandle;
use std::{env, fs, io, sync, thread, time, vec};
use walkdir::WalkDir;
const TEMP_NAME: &'static str = ".brtmp";

struct FileDate {
  name: String,
  date: String,
}

fn main() -> Result<(), io::Error> {
  let start_time = time::SystemTime::now();
  let (is_go, is_verbose, is_sort, glob) = handle_args();
  if !is_verbose {
    println!("Terminal printing disabled, -v to enable.")
  }
  let mut num_files = sync::atomic::AtomicUsize::new(0);
  let num_threads: usize = thread::available_parallelism()?.get();
  let unique_dirs: Vec<String> = get_directories();
  for (i, dir) in unique_dirs.iter().enumerate() {
    thread::scope(|s| {
      let mut children: Vec<ScopedJoinHandle<Result<(), io::Error>>> = vec![];
      if children.len() >= num_threads {
        for child in children {
          child.join().unwrap()?;
        }
        children = vec![];
      }
      children.push(s.spawn(|| -> Result<(), io::Error> {
        let mut valid_files: Vec<FileDate> = get_files(&dir, &glob);
        {
          *num_files.get_mut() += valid_files.len();
        }
        if is_sort {
          valid_files.sort_by_key(|d| d.date.clone());
        }
        rename_files(valid_files, &dir, is_go, is_verbose)?;
        Ok(())
      }));
      if i == unique_dirs.len() - 1 {
        for child in children {
          child.join().unwrap()?;
        }
      }
      Ok::<(), io::Error>(())
    })?;
  }
  let tot_files = num_files.into_inner() as f64;
  let time_elapsed = time::SystemTime::now()
    .duration_since(start_time)
    .unwrap()
    .as_secs_f64();
  println!(
    "{} files in {} seconds. {:.0} files/sec",
    tot_files,
    time_elapsed,
    tot_files / time_elapsed
  );
  if !is_go {
    println!("This was a practice run. -x to execute renaming. Be careful.");
  } else {
    println!("Renaming executed.");
  };
  if is_sort {println!("Sorted by EXIF date.")};
  println!("glob = \"{}\"", &glob);
  Ok(())
}


fn rename_files(
  valid_files: Vec<FileDate>,
  dir: &String,
  is_go: bool,
  is_verbose: bool,
) -> Result<(), io::Error> {
  let mut file_ext_list: Vec<&str> = vec![];
  for (i, file) in valid_files.iter().enumerate() {
    match file.name.split('.').last() {
      Some(file_ext) => file_ext_list.push(file_ext),
      None => file_ext_list.push(""),
    }
    if is_verbose {
      println!("./{} >t> {}/{}{}", file.name, dir, i, TEMP_NAME);
    }
    if is_go {
      fs::rename(&file.name, format!("{}/{}{}", dir, i, TEMP_NAME))?;
    }
  }

  for i in 0..valid_files.len() {
    let dir_vec: Vec<&str> = dir.split('/').collect();
    let new_name = dir_vec[(dir_vec.len() - 2)..].join("");
    if is_verbose {
      println!(
        "{}/{}{} >r> {}/{}{}.{}",
        dir, i, TEMP_NAME, dir, new_name, i, file_ext_list[i],
      );
    }
    if is_go {
      fs::rename(
        format!("{}/{}{}", dir, i, TEMP_NAME),
        format!("{}/{}{}.{}", dir, new_name, i, file_ext_list[i]),
      )?;
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

fn get_files(dir: &String, glob_str: &String) -> Vec<FileDate> {
    glob(&format!("{}/{}", dir, glob_str)).expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
      .map(|path_buff| path_buff.unwrap().into_os_string().into_string().unwrap())
      .map(|file| FileDate {
        date: match exif::Reader::new()
          .read_from_container(&mut io::BufReader::new(fs::File::open(&file).unwrap()))
        {
          Ok(exif) => match exif.get_field(exif::Tag::DateTime, exif::In::PRIMARY) {
            Some(date) => exif::Field::display_value(date).to_string(),
            None => String::from('0'),
          },
          Err(_) => String::from('0'),
        },
        name: file,
      })
      .collect()
}

fn handle_args() -> (bool, bool, bool, String) {
  let args: Vec<String> = env::args().collect();
  let mut is_execute = false;
  let mut is_verbose = false;
  let mut is_practice = false;
  let mut is_sort = false;
  let mut glob = String::from("*.jpg");

  if args.len() == 1 {
    print_help_and_gtfo()
  };
  for (i, arg) in args.iter().enumerate() {
    if i == 0 {
      continue;
    };
    if arg.starts_with('-') {
      if arg.contains('h') {
        print_help_and_gtfo();
      }
        is_sort = arg.contains('s');
        is_execute = arg.contains('x');
        is_verbose = arg.contains('v');
        is_practice = arg.contains('p');
      if arg == "-g" {
        if args.len() - 1 >= i + 1 {
          glob = args[i + 1].to_string();
        } else {
          println!("\nArguments error: Please supply a string to glob for.");
          print_help_and_gtfo();
        }
      }
    }
  }
  if !is_execute && !is_practice {
      println!("\nArguments error: Specify -p for practice run or -x to execute renaming.");
    print_help_and_gtfo();
  }
  if is_execute && is_practice {
    println!("\nArguments error: Don't mix -x and -p ya dingus!");
    print_help_and_gtfo();
  }
  println!("{} {} {} {}", is_execute, is_verbose, is_sort, glob);
  (is_execute, is_verbose, is_sort, glob)
}

fn print_help_and_gtfo() {
  println!(
    r#"
usage - ./batch_renamer -[xpsvh] -g "glob-string"
 e.g. - ./batch_renamer -xvs -g "*.png"
                  ----
options 
        -x               - Execute renaming. Use with caution.
        -v               - Verbose terminal printing.
        -p               - Practice run. Combine with -v to print what the script will do.
        -g "glob_string" - Optional string to glob files with. Defaults to "*.jpg".
        -s               - Sort by EXIF time and date ascending.
        -h               - Print this screen and exit."#
  );
  std::process::exit(0)
}
