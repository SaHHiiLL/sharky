use std::{fs, process};
use crate::Credentials;

pub fn list_network(input: String) {
    if !input.starts_with("list") {
        println!("Unreadable");
    }

    let content = fs::read_to_string("credentials.json");

    if let Err(e) = content {
        println!("Application Error: {}", e);
        process::exit(1);
    }
    let creds: Vec<Credentials> = serde_json::from_str(content.unwrap()
        .as_str())
        .unwrap();

    for i in 0..creds.len() {
        println!("{}. {}", i, creds.get(1).unwrap().name)
    }
}
