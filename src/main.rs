use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env::var;
use std::error;
use std::error::Error;
use std::fmt;
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

// TODO: figure out how to pull this from a ~/.config/jira-cli/config.toml file
//const CONFIG_PATH: String = format!("{}/.config/jira-cli/config.toml", var("HOME"));
const CONFIG_PATH: &str = "/users/rohitmusti/.config/jira-cli/config.toml";

fn write_config(app_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_content = toml::to_string(app_config)?;
    fs::write(CONFIG_PATH, config_content)?;
    Ok(())
}

fn read_config() -> Result<Config, Box<dyn Error>> {

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
    let mut app_config: Config = match read_config() {
        Ok(config) => config.clone(),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // parse out any arguments passed in by the CLI
    // this is the "Activity Tree" of valid actions a user can do
    let args = ActivityTree::parse();

    // if the user passes in a new jira api token, override the original jira api
    match args.jira_api_token {
        Some(jira_api_token) => {
            println!("jira api token provided ({}), replacing the original config value ({})", jira_api_token, app_config.jira_api_token);
            app_config.jira_api_token = jira_api_token;
            match write_config(&app_config) {
                Ok(..) => {
                    println!("successfully updated the app config stored at {}", CONFIG_PATH);
                },
                Err(err) => {
                    println!("📝📄 error updating the app config stored at {}", CONFIG_PATH);
                    eprintln!("{}", err)
                }

            }
        }
        None => {
            println!(
                "no jira api token provided using the config: {}",
                app_config.jira_api_token
            );
        }
    }

    if args.describe_ticket {
        println!("need to describe a ticket");
    } else {
        println!("do not need to describe a ticket");
    }
}

