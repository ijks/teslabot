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
