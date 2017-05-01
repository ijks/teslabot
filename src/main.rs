extern crate discord;

use std::env;

use discord::Discord;
use discord::model::{Channel, Event, Message};

const CHANNELS: &[&str] = &["bot-testing"];

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
                if let Channel::Public(channel) = discord.get_channel(message.channel_id).unwrap() {
                    if CHANNELS.contains(&channel.name.as_str()) {
                        handle_message(&discord, &message);
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

fn handle_message(discord: &Discord, message: &Message) {
    if message.content == "!hello" {
        let response = format!("Hello, {}!", message.author.name);
        discord.send_message(message.channel_id, &response, "", false);
    }
}
