extern crate discord;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate rand;

#[macro_use]
mod command;
mod errors;
mod respond;

use std::env;

use discord::Discord;
use discord::model::{Channel, Event, Message};
use rand::Rng;

use errors::*;
use command::{Command, Commands};
use respond::{Respond, Response};

const CHANNELS: &[&str] = &["bot-testing"];

lazy_static! {
    static ref COMMANDS: Commands = commands! {
        prefix: '!';

        hello(discord, message, params) => {
            let author = &message.author.name;
            let response = if params.len() == 0 {
                format!("Hello, {}!", author)
            } else {
                format!("Hello from {}, {}!", author, params.join(" "))
            };

            Ok(response)
        }

        coinflip(discord, message, params) => {
            const DEFAULT_OPTIONS: &[&str] = &["heads", "tails"];

            let options = match params.len() {
                0 => DEFAULT_OPTIONS,
                2 => params,
                l => bail!(
                    ErrorKind::UserError(format!("wrong amount of arguments: {}", l))
                ),
            };

            // We just made sure `options` isn't empty, so unwrapping is fine here.
            let result = rand::thread_rng().choose(options).unwrap();

            Ok(result.to_string())
        }
    };
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("login token not set");
    let discord = Discord::from_bot_token(&token)
        .expect("login failed");

    println!("Connecting...");
    let (mut connection, _) = discord.connect().expect("connecting failed");
    println!("Sucessfully connected.");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                // TODO: break more stuff out into functions
                if let Channel::Public(channel) = discord.get_channel(message.channel_id).unwrap() {
                    if !CHANNELS.contains(&channel.name.as_str()) {
                        continue
                    }

                    match COMMANDS.respond(&discord, &message) {
                        Ok(Some(msg)) => {
                            discord.send_message(message.channel_id, &msg, "", false);
                        }
                        Ok(None) => {} // Nothing went wrong, but we're not responding either.
                        Err(Error(ErrorKind::UserError(error), _)) => {
                            let msg = format!("Error: {}", error);
                            discord.send_message(message.channel_id, &msg, "", false);
                        }
                        Err(error) => {
                            println!("Got error: {:?}", error);
                            break;
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(error) => {
                println!("Got error: {:?}", error);
                break;
            }
        }
    }
}
