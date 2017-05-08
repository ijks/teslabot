error_chain! {
    errors {
        ArgumentCount(count: u8) {
            description("an invalid amount of arguments was provided")
            display("invalid argument count: {}", count)
        }

        InvalidArgument(error: String) {
            description("the provided arguments were invalid")
            display("invalid arguments: {}", error)
        }
    }
}
