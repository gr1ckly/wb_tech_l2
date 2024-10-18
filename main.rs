use std::env;
use L2_5::Config;

fn main() {
    let args = env::args().collect();
    let cmd = Config::build(args);
    match cmd {
        Ok(mut cmd) => cmd.run(),
        Err(e) => println!("{}", e)
    }
}
