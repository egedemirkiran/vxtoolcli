use clap::{Parser, Subcommand};
use commands::{discord, whois};
use error::Error;
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

mod commands;
mod error;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Whois(whois::Args),
    Discord(discord::SubCommand),
}

fn main() {
    let args = Cli::parse();

    match args.subcmd {
        SubCommand::Whois(lookup) => {
            if let Some(host) = lookup.host {
                match whois::get_addr_info_by_host(&host) {
                    Ok(ip) => println!("{}: {}", host, ip),
                    Err(_) => eprintln!("Failed to lookup {}", host),
                }
            }
            // Passing an empty string if the IP is not provided
            let ip = lookup.ip.unwrap_or("".to_string());

            match whois::whois_by_ip(&ip) {
                Ok(whois) => {
                    println!("{}", whois);
                }
                Err(err) => match err {
                    Error::Request => eprintln!("Failed to send request"),
                    Error::RateLimit => eprintln!("Rate limit exceeded, try again later"),
                    _ => eprintln!("Unknown error"),
                },
            }
        }
        SubCommand::Discord(action) => match action.subcmd {
            discord::Actions::Delete(delete) => delete.execute(),
            discord::Actions::Send(send) => send.execute(),
        },
    }
}

pub fn send_request<T>(
    url: &str,
    method: Method,
    body: Option<HashMap<&str, &String>>,
    query: Option<HashMap<&str, &str>>,
) -> Result<Option<T>, Error>
where
    T: DeserializeOwned,
{
    let client = reqwest::blocking::Client::new();

    let response = match body {
        Some(body) => client
            .request(method, url)
            .header("Content-Type", "application/json")
            .query(&query)
            .json(&body)
            .send(),
        None => client.request(method, url).send(),
    };

    if let Ok(response) = response {
        if response.status().as_u16() == 404 {
            return Err(Error::NotFound);
        }

        if let Ok(body) = response.text() {
            let result: Result<T, serde_json::Error> =
                serde_json::from_str(body.as_str());

            if let Ok(result) = result {
                return Ok(Some(result));
            }
        } else {
            return Ok(None);
        }
    }

    Err(Error::Request)
}
