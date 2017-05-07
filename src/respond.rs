use std::convert::From;

use discord;
use discord::Discord;
use discord::model::{Message, User};

use errors::*;

pub type Response = Result<Option<String>>;

pub trait Respond {
    // TODO: take self as mutable. Doing this has some complications: it is not
    // possible to (safely) have, say, a static COMMANDS variable.
    fn respond(&self, discord: &Discord, message: &Message) -> Response;
}
