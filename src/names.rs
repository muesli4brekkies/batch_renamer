use crate::{
  args::{get_dir_arg, get_glob_arg},
  state::State,
};
use exif::{Field, In, Reader, Tag};
use glob::{glob, GlobError};
use itertools::Itertools;
use std::{fs::File, io::BufReader, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

pub struct Names {
  pub old: String,
  pub tmp: String,
  pub new: String,
}

impl Names {
  fn get((i, file): (usize, String)) -> Names {
    fn attach_backwards(a: String, w: &str) -> String {
      [w, &a].join("")
    }
    let dir = file.rsplit('/').dropping(1);
    let dir_str = dir.clone().rev().join("/");

    Names {
      new: format!(
        "{}/{}{}.{}",
        dir_str,
        dir.take(2).fold(String::new(), attach_backwards),
        i,
        file.split('.').last().unwrap_or("")
      ),
      tmp: format!("{}{}{}", dir_str, i, ".brtmp"),
      old: file,
    }
  }
}

pub fn dirs() -> Vec<Names> {
  let is_sort = State::get().is_sort;

  WalkDir::new(get_dir_arg())
    .min_depth(2)
    .into_iter()
    .filter_map(|dir| match dir {
      Ok(r) => r.file_type().is_dir().then(|| glob_files(is_sort, r)),
      Err(_) => None,
    })
    .flatten()
    .collect_vec()
}

fn glob_files(is_sort: bool, dir: DirEntry) -> Vec<Names> {
  glob(&[dir_ent_to_string(dir), get_glob_arg()].join("/"))
    .expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
    .map(unwrap_pathbuf)
    .sorted_by_key(|f| match is_sort {
      true => get_exif_date(f),
      false => String::from("0"),
    })
    .enumerate()
    .map(Names::get)
    .collect_vec()
}

fn get_exif_date(file: &String) -> String {
  match Reader::new().read_from_container(&mut BufReader::new(File::open(file).unwrap())) {
    Ok(exif) => exif
      .get_field(Tag::DateTime, In::PRIMARY)
      .map(|date| Field::display_value(date).to_string()),
    Err(_) => None,
  }
  .unwrap_or(String::from('0'))
}

fn unwrap_pathbuf(path: Result<PathBuf, GlobError>) -> String {
  path.unwrap().into_os_string().into_string().unwrap()
}

fn dir_ent_to_string(dir: DirEntry) -> String {
  dir.into_path().as_os_str().to_string_lossy().to_string()
}
