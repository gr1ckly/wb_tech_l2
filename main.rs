mod lib;

use std::error::Error;
use std::io::stdin;
use crate::lib::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = Command::build(args);
    if let Ok(cmd) = command{
        cmd.run();
    } else if let Err(str) = command{
        println!("{}", str);
    }
}