# `cargo grammarly`

[![](https://meritbadge.herokuapp.com/cargo-grammarly)](https://crates.io/crates/cargo-grammarly) [![](https://travis-ci.org/vityafx/cargo-grammarly.svg?branch=master)](https://travis-ci.org/vityafx/cargo-grammarly)

# Warning
The command is still at alpha stage. Do not expect too much.

# Description

Improve the quality of your documentation. Use correct English words and grammar, let literally anyone understand
your documentation, allow anyone to use your code, reduce the amount of time people need to spent to understand
how the crate works and what it does. Good examples are necessary but correct spelling and understandable
explanation worths not less.

To check your code for grammar and spelling mistakes, the [`grammarly crate`](https://crates.io/crates/grammarly) is used.

# How it works

The utility simply grabs all the doc comments (`///`, `//!`, `#![doc = "text"]` and
`#[doc = "text"]`) from your crate's source code and sends it to the grammarly grammar
checking bot using the `grammarly` crate. If there are any mistakes in your texts, they
are printed using the way the `rustc` compiler prints its warning and errors.

The doc comments are parsed using the `syn` and `proc_macro2` crates. These are used
specifically to know where in the code these comments are. Doing it with regular
expressions would waste a lot of time.

# Caveats

## Rate limits

The grammarly service has its [rate limits](https://www.grammarbot.io/quickstart), and on `06/06/19` these are:
    
> Request Limits
>
> Grammar Bot offers the most generous free limits on grammar and spelling check, but it's not unlimited.
> With an API key, you can receive 250 requests/day (~7500/mo) at no cost. Without an API key, requests are
> limited to 100 per day per IP address (~3000/mo).  Contact us for paid options if you need higher volumes.

The crate tries hard to minimize the times it sends the requests to the grammarly service, but make sure you
understand the rate limits and that the utility sometimes may not give you any hints.
    
## Documentation text hints

1. Always put the dot at the end of a sentence.
2. If you are registered on the service, it is better to use your account in the service instead, because it does
not have any rate limits.
3. Don't use the \`\` markdown symbols very often, because the grammarly does not handle them correctly.


# Configuring

The utility works out of the box, however, if you want to use your own API key,
you may want to put it in the `.env` file or as an environment variable as:

    GRAMMARLY_API_KEY=99999999

# Installing and Using

Compile the code as shown in the previous section, then put the `cargo-grammarly` executable in your PATH.

My favorite way of doing this is I have a pre-existing directory in `~/bin` that contains little scripts of mine, that dir is added to my PATH in my `.bashrc` so that it's always available, and then I symlink the release version from where it exists to that directory:

    ln -s [starting directory]/cargo-grammarly/target/release/cargo-grammarly ~/bin/

Once you've done that, because of the way cargo is set up to use third party extensions, in any other Rust project of yours, you should be able to run:

    cargo grammarly

and all the documentation and comments of this crate will be checked for grammar.

# License

[MIT](LICENSE)
