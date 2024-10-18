use std::any::type_name;
use std::cmp::{max, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use regex::Regex;

pub struct Config{
    parameters:Vec<Params>,
    contents: String
}

impl Config {
    pub fn build(mut vec: Vec<String>) -> Result<Self, String> {
        let file_name = vec.pop().unwrap();
        let file_content = std::fs::read_to_string(file_name);
        if let Err(e) = file_content{
            return Err(String::from("Incorrect file name"));
        }
        let mut vec = Vec::from(&vec[1..]);
        let file_content = file_content.unwrap();
        let params = Params::parse_keys(vec);
        match params {
            Ok(res) => Ok(
                Config{
                    parameters: res,
                    contents: file_content
                }
            ),
            Err(e) => Err(e)
        }
    }

    pub fn run(&self){
        let mut content_vec: Vec<String> = self.contents.split("\n").map(|str| String::from(str)).collect();
        content_vec.sort_by(|a, b| Self::sort_words(a, b));
        let mut numeric_mode = false;
        for param in self.parameters.clone(){
            match param {
                Params::COLUMN {column: mut columns } => {
                    for _ in 0..columns.len(){
                        let column = columns.pop().unwrap();
                        &content_vec.sort_by(|a, b| Self::sort_by_column(a, b, numeric_mode, column));
                    }
                },
                Params::NUMERIC => numeric_mode = true,
                Params::REVERSE => content_vec.reverse(),
                Params::UNIQUE => content_vec.dedup()
            }
        }
        for str in content_vec{
            println!("{}", str)
        }
    }

    fn sort_by_column(line1: &String, line2: &String, numeric_mode: bool, column: usize) -> Ordering{
        let line1_vec: Vec<String> = line1.split(" ").map(|str| String::from(str)).collect();
        let line2_vec: Vec<String> = line2.split(" ").map(|str| String::from(str)).collect();
        if line1_vec.len() < column{
            if line2_vec.len() < column{
                Equal
            } else{
                Less
            }
        } else if line2_vec.len() < column {
            Greater
        } else {
            if numeric_mode {
                let num1: Result<i32, _> = line1_vec.get(column - 1).unwrap().parse();
                let num2: Result<i32, _> = line2_vec.get(column - 1).unwrap().parse();
                if let Ok(number1) = num1{
                    if let Ok(number2) = num2{
                        number1.cmp(&number2)
                    } else{
                        Less
                    }
                } else if let Ok(number2) = num2{
                    Greater
                } else{
                    let word1 = line1_vec[column - 1].clone();
                    let word2 = line2_vec[column - 1].clone();
                    Self::sort_words(&word1, &word2)
                }
            } else {
                let word1 = line1_vec[column - 1].clone();
                let word2 = line2_vec[column - 1].clone();
                Self::sort_words(&word1, &word2)
            }
        }
    }

    fn sort_words(word1: &String, word2: &String) -> Ordering{
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        for i in 0..max(chars1.len(), chars2.len()){
            if i >= chars1.len(){
                if i < chars2.len() {
                    return Less;
                }
            } else if i >= chars2.len(){
                return Greater
            } else{
                if chars1[i].cmp(&chars2[i]) != Equal{
                    return chars1[i].cmp(&chars2[i]);
                }
            }
        }
        Equal
    }
}



#[derive(Ord, PartialEq, Eq, PartialOrd, Hash, Clone, Debug)]
pub enum Params{
    NUMERIC,
    COLUMN{column: Vec<usize>},
    REVERSE,
    UNIQUE
}

impl Params{
    pub fn parse_keys(words: Vec<String>) -> Result<Vec<Params>, String>{
        let mut params_set: HashSet<Params> = HashSet::new();
        let column_regex = Regex::new(r"(-k([1-9]\d*)*)$").unwrap();
        let mut coll_vec = Vec::new();
        for parameter in words{
            if column_regex.is_match(&parameter){
                if parameter.len() > 2 {
                    let number = String::from(&parameter[2..]);
                    let number: usize = number.parse().unwrap();
                    coll_vec.push(number);
                } else {
                    return Err(format!("Incorrect parameter: {}", parameter));
                }
            } else if parameter == String::from("-n"){
                params_set.insert(Params::NUMERIC);
            } else if parameter == String::from("-u"){
                params_set.insert(Params::UNIQUE);
            } else if parameter == String::from("-r"){
                params_set.insert(Params::REVERSE);
            } else {
                return Err(format!("Incorrect parameter: {}", parameter));
            }
        }
        if coll_vec.len() > 0{
            params_set.insert(Params::COLUMN {column: coll_vec});
        }
        let mut param_vec: Vec<Params> = params_set.into_iter().collect();
        param_vec.sort();
        Ok(param_vec)
    }
}
