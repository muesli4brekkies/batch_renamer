use std::process::ExitCode;

fn main() -> std::io::Result<ExitCode> {
  match batch_renamer::state::run() {
    Ok(_) => std::io::Result::Ok(ExitCode::from(0)),
    Err(e) => Err(e),
  }
}
