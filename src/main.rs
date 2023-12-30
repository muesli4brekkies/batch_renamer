use std::process::ExitCode;

fn main() -> ExitCode {
  batch_renamer::run();
  std::process::exit(0)
}
