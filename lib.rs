mod model;

use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use regex::{RegexBuilder};
use crate::model::Params;

pub struct Config{
    content: Vec<String>,
    pattern: String,
    params: Params
}

impl Config{
    pub fn build(mut data: Vec<String>) -> Result<Self, String>{
        if data.len() > 2{
            let mut data = Vec::from(&data[1..]);
            let content = fs::read_to_string(data.pop().unwrap());
            let mut file;
            match content{
                Ok(opened_file) => {
                    file = opened_file;
                },
                Err(e) => return Err(format!("Failed to open file: {}", data.pop().unwrap())),
            }
            let pattern = data.pop().unwrap();
            let params = Params::parse_keys(data);
            match params{
                Ok(param) => {
                    Ok(Config{
                        content: file.split("\n").map(|str| {String::from(str)}).into_iter().collect(),
                        pattern: pattern,
                        params: param,
                    })
                },
                Err(e) => Err(format!("{}", e))
            }
        } else{
            Err(String::from("Insufficient number of arguments to execute the command"))
        }
    }

    pub fn run(&mut self){
        let mut num_set: HashSet<usize> = HashSet::new();
        let re = RegexBuilder::new(&self.pattern).case_insensitive(*self.params.get_ignore_case()).build().unwrap();
        for i in 0..self.content.len(){
            if *self.params.get_fixed(){
                if re.is_match(self.content.get(i).unwrap()){
                    num_set.insert(i);
                }
            } else if let Some(t) = re.find(self.content.get(i).unwrap()){
                num_set.insert(i);
            }
        }
        if let Some(num) = self.params.get_context(){
            for i in num_set.clone().iter(){
                num_set = Self::add_context_lines(*i, *num, num_set, self.content.len());
            }
        }
        if let Some(num) = self.params.get_after(){
            for i in num_set.clone().iter(){
                num_set = Self::add_after_lines(*i, *num, num_set, self.content.len());
            }
        }
        if let Some(num) = self.params.get_before(){
            for i in num_set.clone().iter(){
                num_set = Self::add_before_lines(*i, *num, num_set);
            }
        }
        if *self.params.get_invert() == true{
            num_set = Self::invert_nums(num_set, self.content.len())
        }
        if *self.params.get_count() == true{
            println!("{}", num_set.len());
        } else {
            let mut num_vec: Vec<usize> = num_set.into_iter().collect();
            num_vec.sort();
            if *self.params.get_line_num() == true{
                for i in num_vec {
                    println!("{} {}", i + 1, self.content[i]);
                }
            } else {
                for i in num_vec {
                    println!("{}", self.content[i]);
                }
            }
        }
    }

    fn invert_nums(set: HashSet<usize>, len_file: usize) -> HashSet<usize>{
        let mut new_set = HashSet::new();
        for i in 0..len_file {
            let num = set.get(&i);
            match num{
                Some(number) => continue,
                None => {
                    new_set.insert(i);
                }
            }
        }
        new_set
    }

    fn add_context_lines(curr: usize, duration: usize, mut lines: HashSet<usize>, len_file: usize) -> HashSet<usize>{
        let start = max(0 as i32, curr as i32 - duration as i32);
        if start < 0{
            let start:usize = 0;
        }
        let start = start as usize;
        for i in start..min(len_file, curr + duration){
            lines.insert(i);
        }
        lines
    }

    fn add_before_lines(curr: usize, duration: usize, mut lines: HashSet<usize>) -> HashSet<usize>{
        let start = max(0 as i32, curr as i32 - duration as i32);
        if start < 0{
            let start:usize = 0;
        }
        let start = start as usize;
        for i in start..curr{
            lines.insert(i);
        }
        lines
    }

    fn add_after_lines(curr: usize, duration: usize, mut lines: HashSet<usize>, len_file: usize) -> HashSet<usize>{
        for i in (curr + 1)..(min(len_file, curr + duration) + 1){
            lines.insert(i);
        }
        lines
    }
}