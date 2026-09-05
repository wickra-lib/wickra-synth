//! The `wickra-synth` reference CLI.
//!
//! Loads a `GenSpec` (from a file or the quick-spec flags), generates the batch
//! or streamed output through `wickra-synth-core`, and prints it as text, JSON, or CSV.

mod args;
mod run;

use args::Args;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    match run::run(&args) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("wickra-synth: {err}");
            ExitCode::FAILURE
        }
    }
}
