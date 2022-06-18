use std:: { fs, io };
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::process;
    /// https://docs.rs/ssh/latest/ssh/

fn main() {

    println!("***********Welcome to Sharky**************");

    let content = fs::read_to_string("credentials.json");

    if let Err(e) = content {
        println!("Application Error: {}", e);
        process::exit(1);
    }
    let x: Vec<Credentials> = serde_json::from_str(content.unwrap()
                                        .as_str())
                                        .unwrap();

    println!("{:?}", &x);
    println!("Which connection would you like to swim in?");
    for creds in &x {
      println!("{}", creds.name);
    }
    let mut cred_name = "linode";
    
    match get_cred(cred_name , &x) {
        Ok(c) => println!("Got success {:?}", c), 
        Err(_) => {
            println!("Error, could not find the sharky in your json file!");
            process::exit(1);
        },
    }
}

fn get_cred<'a>(cred_name: &str, creds: &'a Vec<Credentials>) 
    -> Result<&'a Credentials, ()>
{
    for x in creds{
        if cred_name == x.name {
            return Ok(x);
        }
    }
    Err(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub name: String,
    pub ip: String,
    pub username: Option<String>,
    pub password: String,
}
