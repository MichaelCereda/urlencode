extern crate clap;
extern crate urlencoding;

use std::io::{self, Read};

use clap::Parser;
#[derive(Parser)]
#[command(author="Michael Cereda", version, about="A simple command line utility for URL encoding or decoding stdin.", long_about = None)]

struct Opts {
    #[clap(short, long)]
    /// Decode stdin instead of encoding
    decode: bool,
    #[clap(short, long, default_value = "false")]
    /// Do not append a newline to the output
    nonewline: bool,
    /// Text to parse, this parameter overrides stdin
    text_to_parse: Option<String>,
}

fn read_stdin_or_argument(opts: &Opts) -> String {
    if let Some(text_to_parse) = &opts.text_to_parse {
        return text_to_parse.to_string();
    }
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();

    let input = read_stdin_or_argument(&opts);
    let terminator: &str = if opts.nonewline { "" } else { "\n" };

    if opts.decode {
        print!("{}{}", urlencoding::decode(&input).unwrap(), terminator);
        return Ok(());
    }

    print!("{}{}", urlencoding::encode(&input), terminator);
    Ok(())
}
