use batch_renamer::{get_file_list, get_start_time, rename_files};
fn main() {
  rename_files(get_start_time(), get_file_list(), true);
}
