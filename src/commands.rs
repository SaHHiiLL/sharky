use std::{fs, process};
use crate::Credentials;

pub fn list_network(input: String) {
    if !input.starts_with("list") {
        println!("Unreadable, list is expected in this stage of the application");
        std::process::exit(1);
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

use ssh_cfg::{ SshConfigParser, SshOptionKey };
use tokio::runtime;

pub async fn join_session(input: String, mut creds: &Vec<Credentials>) {
    let ssh_config = SshConfigParser::parse_home().await.unwrap();

    // Print first host config
    if let Some((first_host, host_config)) = ssh_config.iter().next() {
        println!("Host: {}", first_host);

        // Print its configured SSH key if any
        if let Some(identity_file) = host_config.get(&SshOptionKey::IdentityFile) {
            println!("  {} {}", SshOptionKey::IdentityFile, identity_file);
        }
    }

    // Print all host configs
    println!();
    println!("{:#?}", ssh_config);
}