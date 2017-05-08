error_chain! {
    links {
        Command(command::errors::Error, command::errors::ErrorKind)
    }

    foreign_links {
        Discord(::discord::Error);
    }
}
