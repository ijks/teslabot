extern crate discord;

use std::env;

use discord::Discord;
use discord::model::Event;

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
                if message.content == "!hello" {
                    println!("Got !hello from '{}'.", message.author.name);
                    let response = format!("Hello, {}!", message.author.name);
                    discord.send_message(message.channel_id, &response, "", false);
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
