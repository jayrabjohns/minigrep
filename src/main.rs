use std::process;

use clap::Parser;
use minigrep::Args;

fn main() {
    let args = Args::parse();
    if let Err(e) = minigrep::run(&args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
