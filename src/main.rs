extern crate core;

use atty::Stream;
use clap::{ArgEnum, Parser};
use std::process::{exit, ExitCode};

mod custom;
mod liber_primus;
mod lorem_ipsum;
mod utils;

#[derive(Parser)]
/// Terminal application for generating lipsum text.
/// Default usage will generate a string like `Grate Meminit et Praesentibus` using `Liber Primus`
/// which should be suitable for use in a document title.
#[clap(author, version)]
struct Args {
    /// Text source to choose from. Ignored if stdin or `-f` is used.
    #[clap(short, long, arg_enum, default_value = "liber-primus")]
    text_source: TextSource,

    /// File input for the custom source. This has priority to stdin. If not specified, stdin is used.
    #[clap(short, long)]
    file: Option<String>,

    /// Count of words to generate. Default is 5 if text source is not liber-primus.
    #[clap(short, long)]
    words: Option<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum TextSource {
    LiberPrimus,
    LoremIpsum,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let words = args.words.unwrap_or(5);
    let source = args.text_source;
    let input;

    if let Some(file) = args.file {
        input = utils::read_from_file(&file);
        println!("{}", custom::run(&input, words));
        exit(0);
    }

    if atty::isnt(Stream::Stdin) {
        input = utils::read_from_stdin();
        // Needs a minumum of 3 words to produce output
        if input.split_whitespace().count() >= 3 {
            println!("{}", custom::run(&input, words));
            exit(0);
        } else {
            eprintln!(
                "Error: not enough words received from stdin, needs a minimum of \
              three whitespace-seperated words"
            );
            exit(1);
        }
    }

    match source {
        TextSource::LoremIpsum => println!("{}", lorem_ipsum::run(words)),
        TextSource::LiberPrimus => println!("{}", liber_primus::run(words)),
    }

    exit(0);
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
