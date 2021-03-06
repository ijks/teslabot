error_chain! {
    foreign_links {
        Discord(::discord::Error);
    }

    errors {
        // TODO: this could be more granular. something like CommandError, with
        // sub-errors like ArgumentCount &c
        UserError(error: String) {
            description("an error caused by a user")
            display("{}", error)
        }
    }
}
