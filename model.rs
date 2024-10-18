use regex::Regex;

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Params{
    ignore_case: bool,
    fixed: bool,
    context: Option<usize>,
    after: Option<usize>,
    before: Option<usize>,
    invert: bool,
    line_num: bool,
    count: bool,
}

impl Params{
    pub fn parse_keys(words: Vec<String>) -> Result<Params, String>{
        let context_regex = Self::get_regex("C");
        let after_regex = Self::get_regex("A");
        let before_regex = Self::get_regex("B");
        let mut ignore_case = false;
        let mut fixed = false;
        let mut invert = false;
        let mut line_num = false;
        let mut count = false;
        let mut context = None;
        let mut after = None;
        let mut before = None;
        for word in words{
            if context_regex.is_match(&word){
                let number = String::from(&word[2..]);
                let number:Result<usize, _> = number.parse();
                match number{
                    Ok(num) => {
                        if let None = context {
                            context = Some(num);
                        } else if context.unwrap() < num{
                            context = Some(num)
                        }
                    }
                    Err(e) => return Err(format!("Incorrect param: {}", word)),
                }
            } else if after_regex.is_match(&word){
                let number = String::from(&word[2..]);
                let number:Result<usize, _> = number.parse();
                match number{
                    Ok(num) => {
                        if let None = after {
                            after = Some(num);
                        } else if after.unwrap() < num{
                            after = Some(num)
                        }
                    },
                    Err(e) => return Err(format!("Incorrect param: {}", word))
                }
            } else if before_regex.is_match(&word){
                let number = String::from(&word[2..]);
                let number:Result<usize, _> = number.parse();
                match number{
                    Ok(num) => {
                        if let None = before {
                            before = Some(num);
                        } else if before.unwrap() < num{
                            before = Some(num)
                        }
                    },
                    Err(e) => return Err(format!("Incorrect param: {}", word))
                }
            } else if word == String::from("-i"){
                ignore_case = true;
            } else if word == String::from("-F"){
                fixed = true;
            } else if word == String::from("-v"){
                invert = true;
            } else if word == String::from("-n"){
                line_num = true;
            } else if word == String::from("-c"){
                count = true;
            } else{
                return Err(format!("Incorrect param: {}", word))
            }
        }
        Ok(Params{
            ignore_case: ignore_case,
            fixed: fixed,
            context: context,
            after: after,
            before: before,
            invert: invert,
            line_num: line_num,
            count: count
        })
    }

    fn get_regex(param: &str) -> Regex{
        let reg = format!(r"(-{}([1-9]\d*)+)$", param);
        Regex::new(&reg).unwrap()
    }

    pub fn get_ignore_case(&self) -> &bool{
        &self.ignore_case
    }

    pub fn get_fixed(&self) -> &bool{
        &self.fixed
    }

    pub fn get_context(&self) -> &Option<usize>{
        &self.context
    }

    pub fn get_after(&self) -> &Option<usize>{
        &self.after
    }

    pub fn get_before(&self) -> &Option<usize>{
        &self.before
    }

    pub fn get_invert(&self) -> &bool{
        &self.invert
    }

    pub fn get_line_num(&self) -> &bool{
        &self.line_num
    }

    pub fn get_count(&self) -> &bool{
        &self.count
    }
}
