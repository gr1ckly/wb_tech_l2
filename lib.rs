use std::cmp::{max, min};
use std::collections::HashMap;
use std::{fs, thread};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Answer{
    elapsed: String,
    result: HashMap<char, i32>
}

pub struct Config{
    threads: usize,
    content: Arc<RwLock<String>>
}

impl Config{
    pub fn build(mut vec: Vec<String>) -> Result<Self, String>{
        if vec.len() >= 2{
            let mut thread_count: usize = 1;
            let file_name = vec.pop().unwrap();
            let vec = Vec::from(&vec[1..]);
            let content = fs::read_to_string(&file_name);
            match content{
                Ok(content) => {
                    if vec.len() == 2{
                        if vec[0] == String::from("-t"){
                            let num: Result<usize, _> = vec[1].parse();
                            match num{
                                Ok(num) => thread_count = num,
                                Err(e) => return Err(format!("Incorrect number of threads: {}", vec[0]))
                            }
                        } else{
                            return Err(format!("Incorrect param: {}", vec[0]));
                        }
                    } else if vec.len() != 0{
                        return Err(format!("Incorrect params: {}", vec.join(" ")));
                    }
                    if thread_count > 0 {
                        Ok(Config {
                            threads: thread_count,
                            content: Arc::new(RwLock::new(content))
                        })
                    } else{
                        Err(format!("Incorrect number of threads: {}", thread_count))
                    }
                },
                Err(e) => Err(format!("Failed to open file: {}", file_name)),
            }
        } else {
            Err(String::from("Incorrect number of arguments"))
        }
    }

    pub fn run(&self){
        let start = Instant::now();
        let content_len = self.content.read().unwrap().len();
        let number_for_threads = content_len / max(self.threads - 1, 1);
        let mut curr_num = 0;
        let symbol_map = Arc::new(Mutex::new(HashMap::new()));
        let mut handle_vec = Vec::new();
        while curr_num < content_len{
            let map_clone = symbol_map.clone();
            let content_clone = self.content.clone();
            let thread = thread::spawn(move || {
                for symbol in content_clone.read().unwrap()[curr_num..min(curr_num + number_for_threads, content_len)].to_lowercase().chars(){
                    if symbol.is_ascii_alphabetic(){
                         let mut map = map_clone.lock().unwrap();
                         let char = map.get(&symbol);
                         match char{
                             Some(char) => {
                                 let new_char = char + 1;
                                 map.insert(symbol, new_char);
                             },
                             None => {
                                 map.insert(symbol, 1);
                             },
                         }
                     }
                }
            });
            handle_vec.push(thread);
            curr_num = min(curr_num + number_for_threads, content_len);
        }
        for handler in handle_vec{
            handler.join().unwrap();
        }
        let duration = start.elapsed().as_secs_f32();
        let res = Answer{
            elapsed: format!("{} s", duration),
            result: symbol_map.lock().unwrap().clone()
        };
        let ans_string = serde_json::to_string(&res).unwrap();
        println!("{}", ans_string);
    }
}