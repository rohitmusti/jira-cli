use clap::Parser;
use std::error;

use std::fmt;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
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

const CONFIG_PATH: &str = "~/.config/jira-cli/config.toml";

fn read_app_config() -> Result<Config, Box<dyn Error>> {
    if !fs::metadata(CONFIG_PATH).is_ok() {
        let error_message = format!(
            "Error 💾 🚫 🔍: could not find a config file at {}, please create one!",
            CONFIG_PATH
        );
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, error_message).into());
    }

    let toml_content = fs::read_to_string(CONFIG_PATH)?;
    let deserialized_config: Config = toml::from_str(&toml_content)?;
    Ok(deserialized_config)
}

fn main() {
    //let app_config: Config;
    match read_app_config() {
        Ok(config) => {
            //app_config = config;
            println!("{:?}", config);
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
    //print!("{:?}", app_config);

    let args = ActivityTree::parse();

    match args.jira_api_token {
        Some(jira_api_token) => {
            print!("jira api token: {}\n", jira_api_token);
        }
        None => {
            print!("no jira api token provided :(\n");
        }
    }

    if args.describe_ticket {
        print!("need to describe a ticket\n");
    } else {
        print!("do not need to describe a ticket\n");
    }
}
