use std::io::stdin;
use crate::model::Params;

mod model;

pub struct Config{
    params: Params,
}

impl Config{
    pub fn build(args: Vec<String>) -> Result<Self, String>{
        let mut args = Vec::from(&args[1..]);
        let params = Params::parse_params(args);
        match params{
            Ok(param) => {
                Ok(Config{
                    params: param
                })
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    pub fn run(&self){
        let mut line = String::new();
        let mut full_str = String::new();
        let input = stdin();
        while input.read_line(&mut line).unwrap() > 0 {
            full_str.push_str(&line.clone());
            line.clear();
        }
        for line in full_str.split("\n"){
            let mut curr_vec: Vec<String> = line.split(&self.params.get_delimiter()).map(|str| String::from(str)).collect();
            if curr_vec.len() > 1{
                match self.params.get_fields(){
                    Some(vec ) => {
                        for col in vec{
                            if col - 1 < curr_vec.len(){
                                print!("{} ", curr_vec[col - 1]);
                            }
                        }
                        print!("\n")
                    },
                    None => println!("{}", line),
                }
            } else if self.params.get_separated(){
                continue;
            } else {
                println!("{}", line);
            }
        }
    }
}