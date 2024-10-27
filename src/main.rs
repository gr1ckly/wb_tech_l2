use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args: Vec<String> = Vec::from(&args[1..]);
    if args.len() > 0 {
        let url = args.pop().unwrap();
        let mut file_name;
        match args.pop(){
            Some(user_file_name) => file_name = user_file_name,
            None => file_name = url.clone(),
        }
        if args.len() == 0 {
            if !Path::new(&file_name).exists() {
                println!("Start loading...");
                let reqwest = reqwest::get(url).await;
                match reqwest {
                    Ok(response) => {
                        match response.status() {
                            StatusCode::OK => save_to_file(response.text().await.unwrap(), file_name).await,
                            other => println!("Couldn't get data: {}", other),
                        }
                    },
                    Err(e) => println!("{}", e.to_string()),
                }
            } else {
                println!("File {} already exists", file_name);
            }
        } else {
            println!("There have been too many arguments")
        }
    } else {
        println!("You specified insufficient arguments to execute the command")
    }
}

async fn save_to_file(data: String, file_name: String){
    println!("Loading..");
    match File::create(&file_name){
        Ok(mut file) => {
            file.write_all(data.as_bytes()).unwrap();
            println!("The file was successfully uploaded to {}", file_name);
        },
        Err(e) => println!("An error occurred while creating the file {}: {}", file_name, e.to_string())
    }
}
