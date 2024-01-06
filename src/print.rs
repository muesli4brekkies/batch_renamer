use crate::{
  args::{get_dir_arg, get_glob_arg},
  state::State,
};
use std::time::Instant;

pub struct ArgErrors {
  pub help: ArgError,
  pub arg_clash: ArgError,
  pub no_run: ArgError,
}

pub const ERRORS: ArgErrors = ArgErrors {
  help: ArgError("\nHELP:\n\n"),
  arg_clash: ArgError("\nERROR: Don't mix -x and -p ya dingus!\n\n"),
  no_run: ArgError("\nERROR: Need -x or -p to run\n\n"),
};

pub struct ArgError(&'static str);

impl ArgError {
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

pub fn info(start_time: Instant, num_files: f32, state: State) {
  let time_elapsed = start_time.elapsed().as_secs_f32();

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
