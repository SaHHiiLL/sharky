use std:: { fs, io };
use std::io::Read;
use serde::{Deserialize, Serialize};

/// https://docs.rs/ssh/latest/ssh/

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let content = fs::read_to_string(input.trim());

    if let Err(e) = content {
        println!("Application Error: {}", e);
        return;
    }
    let x: Vec<Credentials> = serde_json::from_str(content.unwrap().as_str()).unwrap();

    println!("{:?}", x)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub name: String,
    pub ip: String,
    pub username: Option<String>,
    pub password: String,
}
