use std::env;
use L2_6::Config;

fn main() {
    let args = env::args().collect();
    let cmd = Config::build(args);
    match cmd{
        Ok(cmd) => cmd.run(),
        Err(e) => println!("{}", e)
    }
}
