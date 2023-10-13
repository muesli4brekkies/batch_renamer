use glob::glob;

fn main() {
        let mut valid_files: Vec<String> = Vec::new();
    for entry in glob("./**/*.jpg").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => valid_files.push(path.into_os_string().into_string().unwrap()),
            Err(e) => println!("{:?}", e),
        }
    }
    for file in valid_files {
        let dir :Vec<&str>= file.split("/").collect();
        println!("{:?}",dir);
    }
}
