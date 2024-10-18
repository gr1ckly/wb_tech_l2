fn main() {
    let res = unpack(String::from("\\455"));
    match res{
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e)
    }
}

fn unpack(line: String) -> Result<String, &'static str>{
    let mut new_line = String::new();
    let mut prev_char = None;
    let mut  shielding = false;
    for ch in line.chars(){
        if shielding{
            prev_char = Some(ch);
            new_line.push(ch);
            shielding = false;
        }else {
            if ch.eq(&"\\".chars().nth(0).unwrap()) {
                shielding = true;
            } else if ch.is_numeric(){
                if let Some(prev) = prev_char{
                    let number = (ch as usize) - 48;
                    if number == 0{
                        new_line.pop();
                    } else{
                        for _ in 1..number{
                            new_line.push(prev);
                        }
                    }
                    prev_char = None;
                } else {
                    return Err("Incorrect string");
                }
            } else {
                new_line.push(ch);
                prev_char = Some(ch);
            }
        }
    }
    Ok(new_line)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_empty_line(){
        assert_eq!(unpack(String::from("")).unwrap(), String::from(""));
    }

    #[test]
    fn test_standart_line(){
        assert_eq!(unpack(String::from("a4bc2d5e")).unwrap(), String::from("aaaabccddddde"));
    }

    #[test]
    fn test_only_digit(){
        assert_eq!(unpack(String::from("45")), Err("Incorrect string"));
    }

    #[test]
    fn test_standart_shielding(){
        assert_eq!(unpack(String::from("qwe\\4\\5")).unwrap(), String::from("qwe45"));
    }

    #[test]
    fn test_digit_in_the_end(){
        assert_eq!(unpack(String::from("qwe455")), Err("Incorrect string"));
    }

    #[test]
    fn test_without_digit(){
        assert_eq!(unpack(String::from("abcd")).unwrap(), String::from("abcd"));
    }

    #[test]
    fn test_digit_start(){
        assert_eq!(unpack(String::from("4dgsa")), Err("Incorrect string"));
    }

    #[test]
    fn test_only_slush(){
        assert_eq!(unpack(String::from("\\\\\\\\\\")).unwrap(), String::from("\\\\"));
    }

    #[test]
    fn test_with_null(){
        assert_eq!(unpack(String::from("qwe0")).unwrap(), String::from("qw"));
    }

    #[test]
    fn test_whitespaces(){
        assert_eq!(unpack(String::from("qw 5\n2")).unwrap(), String::from("qw     \n\n"));
    }

    #[test]
    fn test_slush_unpack(){
        assert_eq!(unpack(String::from("qwe\\\\3")).unwrap(), String::from("qwe\\\\\\"));
    }
}