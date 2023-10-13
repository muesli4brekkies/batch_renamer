use glob::glob;
use std::collections::HashSet;
use std::{env, fs};
use walkdir::WalkDir;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut unique_dirs: HashSet<String> = HashSet::new();
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        unique_dirs.insert(String::from(entry.path().to_string_lossy()));
    }
    for dir in unique_dirs {
        let mut valid_files: Vec<String> = Vec::new();
        for entry in glob(&format!("{}/*.jpg", dir)).unwrap() {
            match entry {
                Ok(path) => valid_files.push(path.into_os_string().into_string().unwrap()),
                Err(e) => println!("{:?}", e),
            }
        }
        let mut i = 0;
        for file in &valid_files {
            let dir_vec: Vec<&str> = file.split('/').collect();
            let fd_len = dir_vec.len();
            let file_dir = &dir_vec[..(fd_len - 1)].join("/");
            println!("{} >t> {}/{}.tmp", file, file_dir, i);
            if args.len() > 1 && args[1] == "GO" {
                fs::rename(file, format!("{}/{}.tmp", file_dir, i))?;
            }
            i += 1;
        }
        i = 0;
        for file in &valid_files {
            let dir_vec: Vec<&str> = file.split('/').collect();
            let fd_len = dir_vec.len();
            if fd_len > 3 {
                let file_name = &dir_vec[(fd_len - 3)..(fd_len - 1)].join("");
                let file_dir = &dir_vec[..(fd_len - 1)].join("/");
                println!(
                    "{}/{}.tmp >r> {}/{}{}.jpg",
                    file_dir, i, file_dir, file_name, i
                );
                if args.len() > 1 && args[1] == "GO" {
                    fs::rename(
                        format!("{}/{}.tmp", file_dir, i),
                        format!("{}/{}{}.jpg", file_dir, file_name, i),
                    )?;
                }
            }
            i += 1;
        }
    }
    if args.len() == 1 {
        println!("Pass \"GO\" as an argument to execute renaming")
    };
    Ok(())
}
