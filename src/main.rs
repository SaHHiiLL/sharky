mod commands;

use std::{fs, io, io::Read, io::Write};
use std::fmt::format;
use std::future::Future;
use std::process::Command;



use serde::{ Serialize, Deserialize };
use tokio::runtime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub name: String,
    pub ip: String,
    pub username: Option<String>,
    pub password: String,
}

pub fn main() {
    let file_content = fs::read_to_string("credentials.json");

    //exit if could not find credentials.json
    if let Err(e) = file_content {
        println!("Could not load file! {:?}", e);
        std::process::exit(1);
    }

    let mut creds: Vec<Credentials> = serde_json::from_str(
        file_content.unwrap().as_str()
    ).expect("Could not parse json");

    for i in 0..creds.len() {
        println!("{}. {}", i, creds.get(i).unwrap().name)
    }

    infinite_input_asker(creds)

}

fn infinite_input_asker(mut creds: Vec<Credentials>){
    loop {
        print!("Sharky~> ");
        let mut input = String::new();

        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read line");

        parse_command(input.trim().to_string(), &mut creds);
    }
}

enum ParseResponse {
    VALID, INVALID
}
fn parse_command(input: String, x: &mut Vec<Credentials>) {
    let input: Vec<&str>= input.split("\\s+").collect();
    // if input.starts_with("list") {
    //     commands::list_network(input);
    // }else if input.starts_with("join") {
    //     let rt = runtime::Builder::new_current_thread().build().unwrap();
    //     rt.block_on(commands::join_session(input, x))
    // }else if input.starts_with("create"){
    //     println!("Not yet implemented")
    // }else {
    //     println!("not a valid command!")
    // }

    let i = *input.iter().next().unwrap();



    match i {
        "list" => commands::list_network(i.to_string()),
        "join" => block_future(commands::join_session(i.to_string(), x)),
        "create" => todo!(),
        _ => {eprintln!("Not a Valid command")}
    };
}

fn block_future<F: Future>(future: F)  {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(future);
}
