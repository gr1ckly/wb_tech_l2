use std::env::set_current_dir;
use std::io::Read;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{ChildStdout, Command, Output, Stdio};
use nix::sys::wait;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult};

#[derive(Clone)]
pub enum RuntimeMode{
    FORK,
    EXEC
}

pub struct Comm{
    command: String,
    params: Option<String>
}

impl Comm{
    pub fn parse_command(command: String) -> Result<Self, String>{
        let command_vec:Vec<&str> = command.split(" ").collect();
        if command_vec.len() > 0{
            let mut params: Option<String> = None;
            if command_vec[0] == "cd" || command_vec[0] == "pwd" || command_vec[0] == "echo" || command_vec[0] == "kill" || command_vec[0] == "ps"{
                if command_vec.len() > 1{
                    let new_str =  String::from(command[command_vec[0].len()..].trim());
                    params = Some(new_str);
                }
                Ok(
                    Comm{
                        command: String::from(command_vec[0]),
                        params: params
                    }
                )
            } else {
                Err(format!("Incorrect command: {}", command_vec[0]))
            }
        } else {
            Err(String::from("Command not found"))
        }
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }

    pub fn get_args(&self) -> Option<String> {
        self.params.clone()
    }
}

pub struct Config{
    commands: Vec<Comm>,
    pipes: bool,
    runtime_mode: RuntimeMode
}

impl Config {
    pub fn build(input: &String, mode: RuntimeMode) -> Result<Self, String> {
        let commands: Vec<&str> = input.split("|").collect();
        if commands.len() > 0 {
            let mut pipes = false;
            let mut comm_vec = Vec::new();
            if commands.len() > 1 {
                pipes = true;
            }
            for command in commands {
                let result = Comm::parse_command(String::from(command.trim()));
                match result {
                    Ok(res) => comm_vec.push(res),
                    Err(e) => return Err(e)
                };
            }
            Ok(
                Config {
                    commands: comm_vec,
                    pipes: pipes,
                    runtime_mode: mode
                }
            )
        } else {
            Err(String::from("No commands have been entered"))
        }
    }

    pub fn execute(&self) {
        let mut answer: Option<String> = None;
        if self.pipes {
            let mut prev_command: Option<ChildStdout> = None;
            for command in &self.commands{
                if command.get_command() == String::from("cd"){
                    let mut path: Option<PathBuf> = None;
                    if let Some(mut prev_out) = prev_command.take(){
                        let mut str_path = String::new();
                        prev_out.read_to_string(&mut str_path);
                        path = Some(PathBuf::from(str_path));
                    } else if let Some(args) = command.get_args().take(){
                        path = Some(PathBuf::from(args));
                    }
                    if let Some(path) = path {
                        let res = set_current_dir(&path);
                        match res{
                            Ok(_) => println!("{}", path.to_str().unwrap()),
                            Err(e) => eprintln!("{}", e),
                        }
                    } else{
                        println!("Path not found");
                        break
                    }
                } else {
                    let mut curr_command = Command::new(command.get_command());
                    if let Some(args) = command.get_args() {
                        curr_command.arg(args);
                    }
                    curr_command.stdout(Stdio::piped());
                    if let Some(prev_out) = prev_command.take() {
                        curr_command.stdin(Stdio::from(prev_out));
                    } else {
                        curr_command.stdin(Stdio::piped());
                    }
                    let handle = curr_command.spawn();
                    prev_command = Some(handle.unwrap().stdout.unwrap());
                }
            }
            let mut out_string = String::new();
            prev_command.take().unwrap().read_to_string(&mut out_string);
            answer = Some(out_string);
        } else {
            if self.commands[0].get_command() == String::from("cd"){
                let mut path: Option<PathBuf> = None;
                if let Some(args) = self.commands[0].get_args().take(){
                    path = Some(PathBuf::from(args));
                }
                if let Some(path) = path {
                    let res = set_current_dir(&path);
                    match res{
                        Ok(_) => println!("{}", path.to_str().unwrap()),
                        Err(e) => eprintln!("{}", e),
                    }
                } else{
                    println!("Path not found");
                }
            } else {
                match self.runtime_mode {
                    RuntimeMode::FORK => unsafe {
                        let mut command = Command::new(self.commands[0].get_command());
                        if let Some(args) = self.commands[0].get_args() {
                            command.arg(args);
                        }
                        match fork() {
                            Ok(ForkResult::Parent { child, .. }) => {
                                if let Err(e) = wait() {
                                    answer = Some(e.to_string());
                                }
                            },
                            Ok(ForkResult::Child) => {
                                answer = Some(command.exec().to_string());
                            },
                            Err(e) => {
                                answer = Some(e.to_string());
                            },
                        }
                    },
                    RuntimeMode::EXEC => {
                        let mut command = Command::new(self.commands[0].get_command());
                        if let Some(args) = self.commands[0].get_args() {
                            command.arg(args);
                        }
                        let res = command.spawn().unwrap().wait();
                        if let Err(e) = res {
                            println!("{}", e.to_string())
                        }
                    }
                }
            }
        }
        if let Some(str) = answer{
            println!("{}", str);
        }
    }
}