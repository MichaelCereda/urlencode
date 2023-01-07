extern crate clap;
extern crate urlencoding;

use std::io::{self, Read};

use clap::Parser;
#[derive(Parser)]
#[command(author="Michael Cereda", version, about="A simple command line utility for URL encoding or decoding stdin.", long_about = None)]

struct Opts {
    #[clap(short, long)]
    decode: bool,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    if opts.decode {
        println!("{}", urlencoding::decode(&input).unwrap());
    } else {
        println!("{}", urlencoding::encode(&input));
    }

    Ok(())
}
