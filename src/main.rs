use std:: { fs, io };
use std::io::Read;
use serde::{Deserialize, Serialize};

/// https://docs.rs/ssh/latest/ssh/

fn main() {
    let mut file_name = String::new();

    let file_name = io::stdin().read_line(&mut file_name).expect("Fail to read line");
    let content = fs::read_to_string(file_name);

    if let Err(e) = content {
        println!("Application Error: {}", e);
        return;
    }

    let x: Vec<Credentials> = serde_json::from_str(content.unwrap().as_str()).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub name: String,
    pub ip: String,
    pub username: Option<String>,
    pub password: String,
}
