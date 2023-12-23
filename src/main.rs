use glob::glob;
use std::{env, fs, io, sync, thread, time, vec};
use walkdir::WalkDir;
const TEMP_NAME: &'static str = ".brtmp";

#[derive(Debug)]
struct FileDate {
  name: String,
  date: String,
}

fn main() -> Result<(), io::Error> {
  let start_time = time::SystemTime::now();
  let (is_execute, is_verbose, is_sort, glob) = handle_args();
  if !is_verbose {
    println!("Terminal printing disabled, -v to enable.")
  }
  let num_files = sync::atomic::AtomicUsize::new(0);
  let num_threads: usize = thread::available_parallelism()?.get();
  let unique_dirs: Vec<String> = get_directories();
  let jobs_list = unique_dirs
    .iter()
    .flat_map(|dir| {
      get_files(dir, &glob, is_sort)
        .iter()
        .enumerate()
        .map(|(i, file)| {
          let dir_vec: Vec<&str> = dir.split('/').collect();
          (
            file.name.to_string(),
            format!("{}/{}{}", &dir, i, TEMP_NAME),
            format!(
              "{}/{}{}.{}",
              dir,
              dir_vec[(dir_vec.len() - 2)..].join(""),
              i,
              file.name.split('.').last().unwrap_or("")
            ),
          )
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let tot_files = *&jobs_list.len() as f64;

  thread::scope(|s| {
    jobs_list.chunks(num_threads).for_each(|chunk| {
      chunk
        .iter()
        .map(|job| s.spawn(|| rename_files(&job.0, &job.1, is_execute, is_verbose)))
        .for_each(|h| h.join().unwrap());
    });
    jobs_list.chunks(num_threads).for_each(|chunk| {
      chunk
        .iter()
        .map(|job| s.spawn(|| rename_files(&job.1, &job.2, is_execute, is_verbose)))
        .for_each(|h| h.join().unwrap());
    })
  });
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
  if !is_execute {
    println!("This was a practice run. -x to execute renaming. Be careful.");
  } else {
    println!("Renaming executed.");
  };
  if is_sort {
    println!("Sorted by EXIF date.")
  };
  println!("glob = \"{}\"", &glob);
  Ok(())
}

fn rename_files(from: &String, to: &String, is_execute: bool, is_verbose: bool) {
  if is_verbose {
    println!("{} >> {}", from, to);
  }
  if is_execute {
    fs::rename(from, to).unwrap();
  }
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

fn get_files(dir: &String, glob_str: &String, is_sort: bool) -> Vec<FileDate> {
  let mut files = glob(&format!("{}/{}", dir, glob_str))
    .expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
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
    .collect::<Vec<FileDate>>();
  if is_sort {
    files.sort_by_key(|f| f.date.clone())
  }
  files
}

fn args_contain(c: &str, args: &[(usize, String)]) -> bool {
  args
    .iter()
    .any(|arg| arg.1.starts_with('-') && arg.1.contains(c))
}

fn get_glob(args: &[(usize, String)]) -> String {
  args
    .iter()
    .map(|f| {
      if f.1 == "-g" {
        match args.get(f.0) {
          Some(r) => r.1.to_string(),
          None => String::from(""),
        }
      } else {
        String::from("")
      }
    })
    .collect::<String>()
}

fn handle_args() -> (bool, bool, bool, String) {
  let default = String::from("*.jpg");
  let args = &env::args().enumerate().collect::<Vec<(usize, String)>>()[1..];
  let is_execute = args_contain("x", args);
  let is_verbose = args_contain("v", args);
  let is_practice = args_contain("p", args);
  let is_glob = args_contain("g", args);
  let is_sort = args_contain("s", args);
  if args.is_empty() {
    print_help_and_gtfo()
  };
  let glob = if is_glob { get_glob(args) } else { default };

  if !is_execute && !is_practice {
    println!(
      "\nArguments error: Specify -p for practice run (recommended) or -x to execute renaming."
    );
    print_help_and_gtfo();
  }
  if is_execute && is_practice {
    println!("\nArguments error: Don't mix -x and -p ya dingus!");
    print_help_and_gtfo();
  }
  (is_execute, is_verbose, is_sort, glob.to_string())
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
