use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::collections::HashMap;

use crate::send_request;

#[derive(Parser)]
pub struct SubCommand {
    #[command(subcommand)]
    pub subcmd: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
    Delete(DeleteArgs),
    Send(SendArgs),
}

#[derive(clap::Args)]
pub struct DeleteArgs {
    pub webhook_url: String,
}

#[derive(clap::Args)]
pub struct SendArgs {
    pub message: String,
    pub webhook_url: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

impl DeleteArgs {
    pub fn execute(&self) {
        let response: Result<Option<()>, crate::Error> =
            send_request(&self.webhook_url, reqwest::Method::DELETE, None, None);

        match response {
            Ok(_) => println!("Webhook deleted"),
            Err(error) => match error {
                crate::Error::NotFound => println!("Discord webhook not found"),
                _ => println!("Failed to delete webhook: {:?}", error.to_string()),
            },
        }
    }
}

#[derive(Deserialize)]
struct WebhookResponse {
    channel_id: String,
}

impl SendArgs {
    pub fn execute(&self) {
        let mut body = HashMap::new();
        let mut query = HashMap::new();
        body.insert("content", &self.message);

        if let Some(username) = &self.username {
            body.insert("username", username);
        }

        if let Some(avatar_url) = &self.avatar_url {
            body.insert("avatar_url", avatar_url);
        }

        query.insert("wait", "true");
        let response: Result<Option<WebhookResponse>, crate::Error> = send_request(
            &self.webhook_url,
            reqwest::Method::POST,
            Some(body),
            Some(query),
        );

        match response {
            Ok(response) => {
                if let Some(response) = response {
                    println!("Message sent to channel: {}", response.channel_id);
                }
            }
            Err(error) => match error {
                crate::Error::NotFound => eprintln!("Discord webhook not found"),
                _ => eprintln!("Failed to send message: {:?}", error.to_string()),
            },
        }
    }
}
