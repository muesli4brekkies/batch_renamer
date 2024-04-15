use crate::{
  args::{get_dir_arg, get_glob_arg},
  state::State,
};

use itertools::Itertools;
use std::{fs::File, io::BufReader, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn get_names() -> Vec<Names> {
  let is_sort = State::get().is_sort;

  WalkDir::new(get_dir_arg())
    .min_depth(2)
    .into_iter()
    .filter_map(|dir| match dir {
      Ok(r) => r.file_type().is_dir().then(|| glob_files(is_sort, r)),
      Err(_) => None,
    })
    .flatten()
    .collect()
}

fn glob_files(is_sort: bool, dir: DirEntry) -> Vec<Names> {
  glob::glob(&format!("{}/{}", dir_ent_to_string(dir), get_glob_arg()))
    .expect("Bad glob pattern! Try something like \"*.jpg\" or similar")
    .map(unwrap_pathbuf)
    .sorted_by_key(|f| match is_sort {
      true => get_exif_date(f),
      false => String::from("0"),
    })
    .enumerate()
    .map(Names::construct)
    .collect()
}

fn get_exif_date(file: &str) -> String {
  exif::Reader::new()
    .read_from_container(&mut BufReader::new(File::open(file).unwrap()))
    .ok()
    .and_then(|e| {
      e.get_field(exif::Tag::DateTime, exif::In::PRIMARY)
        .map(|m| m.display_value().to_string())
    })
    .unwrap_or(String::from('0'))
}

fn unwrap_pathbuf(path: Result<PathBuf, glob::GlobError>) -> String {
  path.unwrap().into_os_string().into_string().unwrap()
}

fn dir_ent_to_string(dir: DirEntry) -> String {
  dir.into_path().as_os_str().to_string_lossy().to_string()
}

pub struct Names {
  pub old: String,
  pub tmp: String,
  pub new: String,
}
impl Names {
  fn construct((i, file): (usize, String)) -> Names {
    let dir = file.rsplit('/').dropping(1);
    let dir_str = dir.clone().rev().join("/");

    Names {
      new: format!(
        "{}/{}{}.{}",
        dir_str,
        {
          let (a, b) = dir.take(2).collect_tuple().unwrap();
          [b, a].join("")
        },
        i,
        file.split('.').last().unwrap_or_default()
      ),
      tmp: format!("{}{}{}", dir_str, i, ".brtmp"),
      old: file,
    }
  }
}
