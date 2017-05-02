#[macro_export]
macro_rules! commands {
    (
        prefix: $prefix:expr;
        $(
            $name:ident ($discord:ident, $msg:ident, $params:ident) => $body:block
        )*
    ) => {{
        use $crate::respond::{Command, Commands};
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
            -> $crate::respond::Response {
            $body
        }
    }
}
