mod lib;

use std::error::Error;
use crate::lib::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = Config::build(args);
    if let Ok(cmd) = command{
        cmd.run();
    } else if let Err(str) = command{
        println!("{}", str);
    }
}