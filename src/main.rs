use clap::Parser;
use serde::{Deserialize, Serialize};
use std::error;
use std::env::var;
use std::fmt;
use std::error::Error;
use std::fs;
use toml;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    jira_api_token: String,
}

#[derive(Debug, Parser)]
#[clap(
    author = "Rohit Musti",
    version,
    about = "A cli tool for interacting with your Jira board! This is a tool build for funzies and is not part of the official atlassian toolchain."
)]
struct ActivityTree {
    #[clap(short, long)]
    jira_api_token: Option<String>,
    #[clap(short, long)]
    describe_ticket: bool,
}


fn read_app_config() -> Result<Config, Box<dyn Error>> {

    // TODO: figure out how to pull this from a ~.config/jira-cli/config.toml file
    //const CONFIG_PATH: String = format!("{}/.config/jira-cli/config.toml", var("HOME"));
    const CONFIG_PATH: &str = "/users/rohitmusti/.config/jira-cli/config.toml";
    
    // first check if the file actually exists
    if !fs::metadata(CONFIG_PATH).is_ok() {
        let error_message = format!(
            "Error 💾 🚫 🔍: could not find a config file at {}, please create one!",
            CONFIG_PATH
        );
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, error_message).into());
    }

    // read in the file as a string
    let toml_content = fs::read_to_string(CONFIG_PATH)?;

    // deserialze data into the config object
    let deserialized_config: Config = toml::from_str(&toml_content)?;

    Ok(deserialized_config)
}

fn main() {

    // try and grab the config for the CLI app, exit if an error is raised
    let app_config = match read_app_config() {
        Ok(config) => config.clone(),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let args = ActivityTree::parse();

    match args.jira_api_token {
        Some(jira_api_token) => {
            println!("jira api token provided: {}", jira_api_token);
            println!("going to replace the value in the config w/ the provided value");
        }
        None => {
            println!("no jira api token provided using the config: {}", app_config.jira_api_token);
        }
    }

    if args.describe_ticket {
        println!("need to describe a ticket");
    } else {
        println!("do not need to describe a ticket");
    }
}
