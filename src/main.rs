use std::process::exit;

use clap::Parser;
use clask::{Command, run};

fn main() {
    let args = Command::parse();

    if let Err(_) = run(args) {
        println!("Error while performing operation");
        exit(1);
    }
}
