use glob::glob;
use std::thread::JoinHandle;
use std::{env, fs, io, thread};
use walkdir::WalkDir;

fn rename_to_new_name(num_files: usize, dir: &String, is_go: bool) {
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
                .unwrap();
            }
        }
    }
}

fn rename_to_tmp(valid_files: Vec<String>, dir: &String, is_go: bool) {
    for (i, file) in valid_files.iter().enumerate() {
        println!("{} >t> {}/{}.tmp", file, dir, i);
        if is_go {
            fs::rename(file, format!("{}/{}.tmp", dir, i)).unwrap();
        }
    }
}

fn get_directories() -> Vec<String> {
    WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.into_path().to_string_lossy().to_string())
        .collect()
}

fn glob_files(dir: &String) -> Vec<String> {
    glob(&format!("{}/*.jpg", dir))
        .unwrap()
        .map(|e| e.unwrap().into_os_string().into_string().unwrap())
        .collect()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_go: bool = args.len() > 1 && args[1] == "GO";
    let num_threads = thread::available_parallelism()?;
    let unique_dirs = get_directories();
    let mut children = vec![];
    for (num_children, dir) in unique_dirs.into_iter().enumerate() {
        if num_children >= num_threads.get() {
            children
                .into_iter()
                .for_each(|c: JoinHandle<()>| c.join().unwrap());
            children = vec![];
        }
        children.push(thread::spawn(move || {
            let valid_files = glob_files(&dir);
            let num_files = valid_files.len();
            rename_to_tmp(valid_files, &dir, is_go);
            rename_to_new_name(num_files, &dir, is_go);
        }));
    }
    children
        .into_iter()
        .for_each(|c: JoinHandle<()>| c.join().unwrap());
    if is_go {
        println!("Pass \"GO\" as an argument to execute renaming")
    };
    Ok(())
}
