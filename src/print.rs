use crate::{
  args::{get_dir_arg, get_glob_arg},
  state::State,
};
use std::time::SystemTime;

pub struct Errors {
  pub help: Error,
  pub arg_clash: Error,
  pub no_run: Error,
}

pub const ERRORS: Errors = Errors {
  help: Error("\nHELP:\n\n"),
  arg_clash: Error("\nERROR: Don't mix -x and -p ya dingus!\n\n"),
  no_run: Error("\nERROR: Need -x or -p to run\n\n"),
};

pub struct Error(&'static str);

impl Error {
  pub fn print(&self) {
    println!(
      r#"{}usage - ./batch_renamer -h -[vq] -[px] -s -g "glob-string" -d <path>
 e.g. - ./batch_renamer -xvs -g "*.png" -d ./directory
options 
        -v               - Verbose terminal printing.
        -q               - Disable terminal printing entirely. Overrides -v.

        -p               - Practice run. Combine with -v to print what the script will do!
        -x               - Execute renaming. Use with caution.

        -s               - Optional Sort by EXIF timestamp ascending. Default is not to sort, so the files are ordered however the OS picks them up.
        -g "glob_string" - Optional string to glob files with.        Defaults to "*.jpg".
        -d <path>        - Optional path to run search from.          Defaults to directory the binary is run from.
        "#,
      self.0
    );
    std::process::exit(2)
  }
}

pub fn info(start_time: SystemTime, num_files: f32, state: State) {
  let time_elapsed = start_time
    .elapsed()
    .expect("\nTime has gone backwards. :(\n")
    .as_secs_f32();

  println!(
    "{} files in {} seconds. {:.0} files/sec\n{}\n{}\nglob = \"{}\"\nroot dir = \"{}\"",
    num_files,
    time_elapsed,
    num_files / time_elapsed,
    match state.is_exec {
      true => "Renaming executed.",
      false => "This was a practice run. -x to execute renaming. Be careful.",
    },
    match state.is_sort {
      true => "Sorted by EXIF date.",
      false => "NOT sorted",
    },
    get_glob_arg(),
    get_dir_arg()
  );
}
