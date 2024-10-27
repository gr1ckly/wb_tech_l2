use std::env::args;
use std::ffi::CString;
use std::time::Duration;
use clap::Parser;
use L2_10::Config;

#[derive(Debug, Parser)]
#[command(name="telnet")]
struct CLI{
    #[arg(long, default_value = "10s")]
    timeout: String,
    first: String,
    second: u16
}

fn main() {
    let cli = CLI::parse();
    let config = Config::build(&cli.first, cli.second, cli.timeout);
    match config{
        Ok(mut config) => {
            println!("Connected to {}:{}", cli.first, cli.second);
            config.run();
        },
        Err(e) => println!("{}", e.to_string()),
    }
}
