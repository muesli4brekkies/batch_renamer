use std::process::ExitCode;

fn main() -> ExitCode {
  batch_renamer::run();
  ExitCode::from(0)
}
