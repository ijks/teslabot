extern crate discord;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
mod respond;

use std::env;

use discord::Discord;
use discord::model::{Channel, Event, Message};

use respond::{Commands, Respond, Response};

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

            Response::Respond(response)
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

                    if let Some(response) = COMMANDS.respond(&discord, &message) {
                        match response {
                            Response::Respond(msg) => {
                                discord.send_message(message.channel_id, &msg, "", false);
                            }
                            Response::UserError(error) => {
                                let msg = format!("Error: {}", error);
                                discord.send_message(message.channel_id, &msg, "", false);
                            }
                            Response::InternalError(error) => {
                                println!("Got error: {:?}", error);
                                break;
                            }
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
