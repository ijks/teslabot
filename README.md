# Teslabot

This is a Discord bot I'm currently developing. All very early-stages.

I might break out functionality into a library or framework.

# Personal Notes

 These are some personal notes while I'm developing this. If things ever get
 stable, I'll probably remove this. So hello, (possible) paleogitologists!

## Features

Not sure yet what I definitely want to add. Also not sure if I want to focus more
on making a generic library, or lots of features for the bot itself.

## API Design

## Defining a Bot

I'm not sure yet on the API for defining a bot. Currently, all logic is just
dumped in `main()`, but I'd like a more generic way to specify it all. I see two
possible ways to abstract it.

Firstly, there's a type-based approach, where there is one concrete `Bot` type,
probably accompanied by a builder type. Using it would look like this:

```rust
let bot = BotBuilder::new()
    .command_prefix('!')
    .channel_whitelist(&["foo", "bar"])
    .some_other_option(true)
    .build()
    .unwrap()
```

I'm not sure how commands or other triggers would then be defined. I also just
don't feel like the builder pattern is appropriate here.

There's also a trait-based approach, where `Bot` is a trait.

```rust
trait Bot {
    // TBD
}
```

And one could implement the trait manually, or could maybe use a macro.

```rust
bot! {
    commands {
        // definition of commands here, see `commands!`
    }

    triggers {
        regex r"(foo|bar)" (discord, msg, matches) {
            Ok("saw a foobar!")
        }

        event Event::PresenceUpdate { presence, .. } (discord) {
            // check if the user didn't change their nick to someone else's
        }

        // It might actually be a better idea to put commands under triggers as
        // well. But they do have a more rigid interface, and they always return
        // something to the user, as opposed to triggers.
    }
}
```

But too much macro magic might complicate things. It could be better to have
stuff defined using normal syntax.

### Commands

Commands are currently just an fn-pointer that take (mainly) a list of
arguments, and return a `Response`. Firstly, I'm going to remove `Response`,
to be replaced by a good ole' `Result<T>`. Secondly, it would be nice to
define commands more strictly, by specifying all their arguments and,
possibly, types. I'm not sure where this would be on the dynamic-static
scale. Doing a lot at compile time is nice, but users might want to define
their own commands through some interface.

Commands will probably defined by a trait. Something like:

```rust
trait Command {
    type Args;
    fn execute(args: Args) -> Result<String, CommandError>
}
```

And then there would be some kind of trait for `Args` as well, or maybe it would
be constrained as `TryFrom<Message>`. The overall goal is for argument validation
to be generated automatically.

An alternative is for `Command` to just be a type. This would make user-defined
commands easier.
