#![allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
