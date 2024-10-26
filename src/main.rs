use std::io::{stdin, Stdin};
use L2_8::{Config, RuntimeMode};
use L2_8::RuntimeMode::{EXEC, FORK};

fn main(){
    let mut input_string = String::new();
    let stdin = stdin();
    let mode = asker_about_mode(&mut input_string, &stdin);
    input_string.clear();
    if let Some(mode) = mode{
        println!("Enter the command: ");
        while stdin.read_line(&mut input_string).unwrap() > 0{
            if input_string.trim() == String::from("\\quit"){
                break;
            }
            let config = Config::build(&input_string, mode.clone());
            match config{
                Ok(config) => {
                    config.execute();
                },
                Err(e) => println!("{}", e)
            }
            input_string.clear();
            println!("Enter the command: ");
        }
    }
    println!("Completion of work...")
}

fn asker_about_mode(input_string: &mut String, stdin: &Stdin) -> Option<RuntimeMode>{
    let mut curr_mode = None;
    while let None = curr_mode{
        input_string.clear();
        println!("Enter runtime mode (fork, exec):");
        let is_repeat = stdin.read_line(input_string).unwrap();
        if is_repeat > 0{
            match input_string.trim() {
                "fork" => curr_mode = Some(FORK),
                "exec" => curr_mode = Some(EXEC),
                _ => continue,
            }
        } else {
            break
        }
    }
    curr_mode
}