use std::collections::HashMap;
use std::convert::From;

use discord;
use discord::Discord;
use discord::model::{Message, User};

// TODO: remove this enum. We can just use Result with our custom Error type.
#[derive(Debug)]
pub enum Response {
    Respond(String),
    UserError(String),
    InternalError(discord::Error),
}

impl From<Result<String, discord::Error>> for Response {
    fn from(result: Result<String, discord::Error>) -> Response {
        match result {
            Ok(s) => Response::Respond(s),
            Err(e) => Response::InternalError(e),
        }
    }
}

pub trait Respond {
    // TODO: take self as mutable. Doing this has some complications: it is not
    // possible to (safely) have, say, a static COMMANDS variable.
    fn respond(&self, discord: &Discord, message: &Message) -> Option<Response>;
}

pub type Command = fn(&Discord, & Message, &[&str]) -> Response;

pub struct Commands {
    prefix: char,
    commands: HashMap<&'static str, Command>,
}

impl Commands {
    pub fn new(prefix: char, commands: HashMap<&'static str, Command>) -> Self {
        Commands { prefix, commands }
    }
}

impl Respond for Commands {
    fn respond(&self, discord: &Discord, message: &Message) -> Option<Response> {
        let text = &message.content;
        if !text.starts_with(self.prefix) {
            return None;
        }

        let split: Vec<_> = text.split_whitespace().collect();
        let command = split[0];
        let params = &split[1..];

        let response = match self.commands.get(&command[1..]) {
            Some(cmd) => cmd(discord, message, params),
            None => Response::UserError(format!("unrecognized command: '{}'", command))
        };

        Some(response)
    }
}
