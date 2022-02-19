extern crate core;

use std::fs::File;
use std::io::{self, Read};
use atty::Stream;
use clap::{ArgEnum, Parser};

mod liber_primus;
mod lorem_ipsum;
mod custom;

/// Terminal application for generating lipsum text.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// What mode to run the program in
    #[clap(arg_enum, default_value_t = Source::LiberPrimus)]
    source: Source,

    /// File input for the custom source. If not specified, stdin is used.
    #[clap(short, long, default_value = "")]
    file: String,

    /// Name of the person to greet
    #[clap(short, long, default_value = "0")]
    words: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Source {
    LiberPrimus,
    LoremIpsum,
    Custom,
}

fn main() {
    let args = Args::parse();
    let mut input = String::new();

    if args.source == Source::Custom {
        if atty::isnt(Stream::Stdin) {
            loop {
                match io::stdin().read_line(&mut input) {
                    Ok(len) => if len == 0 {
                        break;
                    }
                    Err(error) => {
                        eprintln!("error: {}", error);
                        return;
                    }
                }
            }
        } else {
            if args.file.is_empty() {
                eprintln!("error: no file specified and stdin is empty");
                return;
            }
            let mut f = File::open(args.file).expect("File not found!");
            f.read_to_string(&mut input).expect("Something went wrong reading the file!");
        }
    }

    match args.source {
        Source::LiberPrimus => println!("{}", liber_primus::run(args.words)),
        Source::LoremIpsum => println!("{}", lorem_ipsum::run(args.words)),
        Source::Custom => println!("{}", custom::run(&*input, args.words)),
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}