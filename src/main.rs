use glob::glob;
use std::thread::JoinHandle;
use std::{env, fs, io, thread};
use walkdir::WalkDir;
fn main() -> io::Result<()> {
    let num_threads = thread::available_parallelism()?;
    let unique_dirs: Vec<_> = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .collect();
    let mut children = vec![];
    for (num_children, dir) in unique_dirs.into_iter().enumerate() {
        if num_children >= num_threads.get() {
            children
                .into_iter()
                .for_each(|c: JoinHandle<()>| c.join().unwrap());
            children = vec![];
        }
        children.push(thread::spawn(move || {
            let args: Vec<String> = env::args().collect();
            let valid_files: &Vec<String> = &glob(&format!("{}/*.jpg", dir.into_path().display()))
                .unwrap()
                .map(|e| e.unwrap().into_os_string().into_string().unwrap())
                .collect();
            for (i, file) in valid_files.iter().enumerate() {
                let dir_vec: Vec<&str> = file.split('/').collect();
                let fd_len = dir_vec.len();
                let file_dir = &dir_vec[..(fd_len - 1)].join("/");
                println!("{} >t> {}/{}.tmp", file, file_dir, i);
                if args.len() > 1 && args[1] == "GO" {
                    fs::rename(file, format!("{}/{}.tmp", file_dir, i)).unwrap();
                }
            }
            for (i, file) in valid_files.iter().enumerate() {
                let dir_vec: Vec<&str> = file.split('/').collect();
                let fd_len = dir_vec.len();
                if fd_len > 2 {
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
                        )
                        .unwrap();
                    }
                }
            }
        }));
    }
    children
        .into_iter()
        .for_each(|c: JoinHandle<()>| c.join().unwrap());
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Pass \"GO\" as an argument to execute renaming")
    };
    Ok(())
}
