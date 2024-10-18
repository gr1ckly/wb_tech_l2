use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::task::Context;
use regex::Regex;

pub struct Command{
    parameter: Option<String>,
    contents: Option<String>
}

impl Command {
    pub fn build(vec: Vec<String>) -> Result<Self, &'static str> {
        if vec.len() >= 2 && vec.len() <= 3 {
            let file = std::fs::read_to_string(vec.get(vec.len() - 1).unwrap());
            let mut contents = None;
            match file {
                Ok(cont) => {
                    contents = Some(cont);
                }
                Err(_) => return Err("Failed to read data from the file")
            };
            let mut arg = None;
            if vec.len() == 3 {
                if vec.get(1).unwrap() != "-c" && vec.get(1).unwrap() != "-l" && vec.get(1).unwrap() != "-w" {
                    return Err("Invalid parameter");
                }
                arg = Some(vec.get(1).unwrap().clone());
            }
            return Ok(Command {
                parameter: arg,
                contents: contents
            });
        }
        Err("Invalid number of command line arguments")
    }

    pub fn run(self) {
        match &self.parameter {
            Some(param) => {
                match param.as_str() {
                    "-c" => println!("{}", Self::count_symbols(&self.contents.unwrap())),
                    "-l" => println!("{}", Self::count_lines(&self.contents.unwrap())),
                    "-w" => println!("{}", Self::count_words(&self.contents.unwrap())),
                    other => println!("Invalid parameter")
                }
            },
            None => println!("{}", Command::count_words(&self.contents.unwrap()))
        }
    }

    fn count_words(contents: &String) -> usize {
        let regex = Regex::new(r"\s+").unwrap();
        let vec: Vec<&str> = regex.split(contents).collect();
        vec.len()
    }

    fn count_symbols(contents: &String) -> usize {
        contents.chars().count()
    }

    fn count_lines(contents: &String) -> usize {
        contents.split("\n").count()
    }
}