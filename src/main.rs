use clap::Parser;

//#[derive(Debug, Deserialize)]
//struct Config {
//    api_token: String,
//}

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

fn main() {
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

