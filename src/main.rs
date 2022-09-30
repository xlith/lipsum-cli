extern crate core;

use std::process::{exit, ExitCode};
use atty::Stream;
use clap::{ArgEnum, Parser};

mod liber_primus;
mod lorem_ipsum;
mod custom;
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

    let file = args.file.unwrap_or("".to_string());
    let source = args.text_source;
    let input;
    let words;

    if file != "" {
        words = 5;
        input = utils::read_from_file(&file);
        println!("{}", custom::run(&*input, words));
        exit(0);
    }

    if atty::isnt(Stream::Stdin) {
        input = utils::read_from_stdin();
        if input.chars().count() > 0 {
            words = 5;
            println!("{}", custom::run(&*input, words));
            exit(0);
        }
    }

    if source == TextSource::LoremIpsum {
        words = 5;
        println!("{}", lorem_ipsum::run(words));
        exit(0);
    }

    if source == TextSource::LiberPrimus {
        words = 0;
        println!("{}", liber_primus::run(words));
        exit(0);
    }

    panic!("Invalid source");
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}