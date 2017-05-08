use std::collections::HashMap;

use discord::Discord;
use discord::model::Message;

use errors;
use respond::{Respond, Response};
use self::errors::*;

// pub type Command = fn(&Discord, &Message, &[&str]) -> Result<String>;

pub trait Command {
    type Args: Argument;

    fn execute(&mut self, discord: &Discord, message: &Message, args: Self::Args)
        -> errors::Result<String>;
}

pub trait Argument {
    fn parse(arguments: &str) -> Result<Self>;
}

impl Argument for () {
    fn parse(arguments: &str) -> Result<Self> {
        ensure!(arguments == "",
            ErrorKind::InvalidArgument("this command takes no arguments"));

        Ok(())
    }
}

impl<T> Argument for T
    where T: FromStr
{
    fn parse(arguments: &str) -> Result<Self> {
        FromStr::from_str(arguments).map_err(Into::into)
    }
}

impl<T, 'a> Argument for &'a [T]
    where T: Argument
{
    fn parse(arguments: &str) -> Result<Self> {
        arguments.split_whitespace().map(Arguments::parse).collect()
    }
}

impl<T> Argument for Option<T>
    where T: Argument
{
    fn parse(arguments: &str) -> Result<Self> {
        match arguments {
            "" => Ok(None),
            arg => Argument::parse(arg).map(Some),
        }
    }
}

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
    fn respond(&self, discord: &Discord, message: &Message) -> Response {
        let text = &message.content;
        if !text.starts_with(self.prefix) {
            return Ok(None);
        }

        let split: Vec<_> = text.split_whitespace().collect();
        let command = split[0];
        let params = &split[1..];

        match self.commands.get(&command[1..]) {
            Some(cmd) => cmd(discord, message, params).map(Some),
            None => bail!(
                ErrorKind::UserError(format!("unrecognized command: '{}'", command))
            ),
        }
    }
}

#[macro_export]
macro_rules! commands {
    (
        prefix: $prefix:expr;
        $(
            $name:ident ($discord:ident, $msg:ident, $params:ident) => $body:block
        )*
    ) => {{
        use $crate::command::{Command, Commands};
        let mut commands = ::std::collections::HashMap::new();
        $(
            command_fn!($name, $discord, $msg, $params, $body);

            // We need to convert the locally defined fn to a function pointer here,
            // because the compiler will complain otherwise.
            let command = $name as Command;
            commands.insert(stringify!($name), command);
        )*
        Commands::new($prefix, commands)
    }}
}

macro_rules! command_fn {
    ($name:ident, $discord:ident, $msg:ident, $params:ident, $body:block) => {
        fn $name($discord: &::discord::Discord, $msg: &::discord::model::Message, $params: &[&str])
            -> $crate::errors::Result<String> {
            $body
        }
    }
}
