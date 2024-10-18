use std::fmt::format;
use regex::Regex;

pub struct Params{
    fields: Option<Vec<usize>>,
    delimiter: String,
    separated: bool
}

impl Params{
    pub fn parse_params(vec: Vec<String>) -> Result<Self, String>{
        let mut fields = None;
        let mut delimiter = String::from("\t");
        let mut separated = false;
        let mut counter = 0;
        let fields_regex = Regex::new(r"(-f(([1-9]\d*)(,)*)+)$").unwrap();
        while counter < vec.len(){
            if fields_regex.is_match(&vec[counter]){
                let numbers = String::from(&vec[counter][2..]);
                let numbers: Vec<String> = numbers.split(",").map(|str| String::from(str)).collect();
                let mut num_vec = Vec::new();
                for number in numbers{
                    let curr_num = number.parse();
                    match curr_num{
                        Ok(num) => num_vec.push(num),
                        Err(e) => return Err(format!("Incorrect column: {}", number)),
                    }
                }
                fields = Some(num_vec);
            } else if vec[counter] == String::from("-d"){
                counter += 1;
                if counter < vec.len(){
                    delimiter = vec[counter].clone();
                } else{
                    return Err(format!("Delimiter has not been entered"));
                }
            } else if vec[counter] == String::from("-s"){
                separated = true;
            } else {
                return Err(format!("Incorrect param: {}", vec[counter]))
            }
            counter += 1;
        }
        Ok(Params{
            fields: fields,
            delimiter: delimiter,
            separated: separated
        })
    }

    pub fn get_fields(&self) -> Option<Vec<usize>>{
        self.fields.clone()
    }

    pub fn get_separated(&self) -> bool{
        self.separated
    }

    pub fn get_delimiter(&self) -> String{
        self.delimiter.clone()
    }
}