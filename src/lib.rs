use std::error::Error;
use std::{io, thread};
use std::io::{stdin, Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
pub struct Config{
    connection: TcpStream,
}

impl Config{
    pub fn build(host: &String, port: u16, timeout: String) -> Result<Self, String>{
        let socket_addresses: Vec<SocketAddr> = format!("{}:{}", host, port).to_socket_addrs().unwrap().into_iter().collect();
        match humantime::parse_duration(&timeout) {
            Ok(duration) => {
                for socket_addr in socket_addresses {
                    match TcpStream::connect_timeout(&socket_addr, duration.clone()) {
                        Ok(connection) => {
                            connection.set_read_timeout(Some(duration.clone()));
                            connection.set_write_timeout(Some(duration.clone()));
                            return Ok(
                            Config {
                                connection: connection,
                            }
                            )
                        },
                        Err(e) => continue,
                    };
                }
                Err(format!("Couldn't connect to {}:{}", host, port))
            },
            Err(e) => Err(e.to_string())
        }
    }

    pub fn run(&mut self) {
        let mut input_string = String::new();
        let mut response = Vec::new();
        let mut stdin = stdin();
        loop{
            if stdin.read_line(&mut input_string).unwrap() > 0 {
                self.connection.write_all(input_string.as_bytes()).unwrap();
                match self.connection.read_to_end(&mut response){
                    Ok(0) => {
                        println!("Sever disconnected");
                        break
                    },
                    Ok(n) => {
                        println!("{}", String::from_utf8_lossy(&response));
                    },
                    Err(e) => {
                        println!("{}", e.to_string());
                        break
                    }
                }
            } else{
                break
            }
            input_string.clear();
            response.clear();
        }
        println!("Completion telnet's work");
    }
}