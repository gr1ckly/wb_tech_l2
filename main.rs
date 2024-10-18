use std::collections::{HashMap, HashSet};
use std::ops::Index;

fn main() {
    let vec = vec!["пятак".to_string(), "пятка".to_string(), "тяпка".to_string(), "листок".to_string(), "слиток".to_string(), "столик".to_string(), "лол".to_string(), "инфо".to_string(), "ФОни".to_string()];
    let res = mapping_string(vec);
    for (key, value) in res{
        let mut line = String::new();
        for str in value{
            line.push_str(&*str);
            line.push_str(" ");
        }
        println!("{}: {}", key, line);
    }
}

fn mapping_string(words: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut ans_map:HashMap<String, Vec<String>> = HashMap::new();
    let mut set_vec = Vec::new();
    let mut val_vec: Vec<String> = Vec::new();
    for mut word in words{
        word = word.to_lowercase();
        let set = HashSet::from_iter(word.chars());
        let index = set_vec.iter().position(|x: &HashSet<char>| x == &set);
        if let Some(ind) = index{
            let anagram = val_vec.get(ind).unwrap().to_string();
            let mut vec = ans_map.get_mut(&anagram);
            match vec{
                Some(result) => result.push(word),
                None => {
                    let mut new_vec = Vec::new();
                    new_vec.push(anagram.clone());
                    new_vec.push(word);
                    ans_map.insert(anagram, new_vec);
                }
            }
        } else{
            set_vec.push(set);
            val_vec.push(word);
        }
    }

    let mut new_map = HashMap::new();

    for (key, mut value) in ans_map{
        value.sort();
        new_map.insert(key, value);
    }
    new_map
}
