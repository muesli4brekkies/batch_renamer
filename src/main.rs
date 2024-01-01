use std::process::ExitCode;

fn main() -> ExitCode {
  batch_renamer::state::run();
  ExitCode::from(0)
}
