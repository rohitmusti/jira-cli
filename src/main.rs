use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::{eprintln, fs, println};
use toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    jira_api_token: String,
    jira_domain: String,
    account_email: String,
}

#[derive(Debug, Parser)]
#[clap(
    author = "Rohit Musti",
    version,
    about = "A cli tool for interacting with your Jira board! This is a tool build for funzies and is not part of the official atlassian toolchain."
)]
struct ActivityTree {
    #[clap(long)]
    jira_api_token: Option<String>,
    #[clap(long)]
    jira_domain: Option<String>,
    #[clap(long)]
    account_email: Option<String>,
    #[clap(short, long)]
    issue_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JiraContent {
    text: String,
}

#[derive(Debug, Deserialize)]
struct JiraContentList {
    content: Vec<JiraContent>,
}

#[derive(Debug, Deserialize)]
struct JiraDescription {
    content: Vec<JiraContentList>,
}

#[derive(Debug, Deserialize)]
struct JiraFields {
    description: Option<JiraDescription>,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct JiraResp {
    fields: JiraFields,
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
            std::process::exit(1);
        }
    };

    // parse out any arguments passed in by the CLI
    // this is the "Activity Tree" of valid actions a user can do
    let args = ActivityTree::parse();

    // if the user passes in a new jira api token, override the original jira api
    match args.jira_api_token {
        Some(ref jira_api_token) => {
            println!(
                "jira api token provided ({}), replacing the original config value ({})",
                jira_api_token, app_config.jira_api_token
            );
            app_config.jira_api_token = jira_api_token.clone();
            match write_config(&app_config) {
                Ok(..) => {
                    println!(
                        "successfully updated the app config stored at {}",
                        CONFIG_PATH
                    );
                }
                Err(err) => {
                    println!(
                        "📝📄 error updating the app config stored at {}",
                        CONFIG_PATH
                    );
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

    if args.issue_id.is_some() {
        println!("need to describe a ticket");

        // construct URL for get
        let url = format!(
            "{}/rest/api/3/issue/{}",
            app_config.jira_domain,
            args.issue_id.unwrap()
        );

        // Use reqwest to send a GET request
        let client = reqwest::blocking::Client::new();

        // Use reqwest to send a GET request with basic authentication
        let response = client
            .get(url)
            .basic_auth(
                app_config.account_email.as_str(),
                Some(app_config.jira_api_token.as_str()),
            )
            .send();

        // get text result from response
        let text_result = response
            .expect("we expected there to be a text value in the response!")
            .text();

        let processed_result: JiraResp = match text_result {
            Ok(text) => {
                // Deserialize the JSON string into a serde_json::Value
                let json_body: Value = serde_json::from_str(&text)
                    .expect("failed to turn the text body into a Value!");

                // Convert the JSON to a pretty-formatted string
                let jira_resp: JiraResp = serde_json::from_value(json_body)
                    .expect("should have gotten a nicely formmated struct");

                jira_resp
            }
            _ => {
                eprintln!("failed to process result from jira's API");
                std::process::exit(1);
            }
        };

        println!("{:?}", processed_result)
    } else {
        println!("do not need to describe a ticket");
    }
}
