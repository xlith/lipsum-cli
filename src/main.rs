extern crate core;

use atty::Stream;
use clap::{Parser, ValueEnum};
use std::process::exit;

mod custom;
mod liber_primus;
mod lorem_ipsum;
mod new_lines;
mod utils;

#[derive(Parser)]
/// Terminal application for generating lipsum text.
/// Default usage will generate a string like `Grate Meminit et Praesentibus` using `Liber Primus`
/// which should be suitable for use in a document title.
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text source to choose from. Ignored if stdin or `-f` is used.
    #[arg(short, long, value_enum, default_value = "liber-primus")]
    text_source: TextSource,

    /// File input for the custom source. This has priority to stdin. If not specified, stdin is used.
    #[arg(short, long)]
    file: Option<String>,

    /// Count of words to generate. Default is 10 if text source is not liber-primus.
    #[arg(short, long)]
    words: Option<usize>,

    /// Count of words after a new line is inserted. By default deactivated.
    /// Zero is the same as deactivated
    #[arg(short = 'l', long)]
    words_per_line: Option<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TextSource {
    LiberPrimus,
    LoremIpsum,
}

fn main() {
    let args = Args::parse();

    let words = args.words.unwrap_or(5);
    let source = args.text_source;
    let words_per_line = args.words_per_line;
    let input;

    if let Some(file) = args.file {
        input = utils::read_from_file(&file);
        print_output(custom::run(&input, words), words_per_line);
        exit(0);
    }

    if atty::isnt(Stream::Stdin) {
        input = utils::read_from_stdin();
        // Needs a minumum of 3 words to produce output
        if input.split_whitespace().count() >= 3 {
            print_output(custom::run(&input, words), words_per_line);
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
        TextSource::LoremIpsum => print_output(lorem_ipsum::run(words), words_per_line),
        TextSource::LiberPrimus => print_output(liber_primus::run(words), words_per_line),
    }

    exit(0);
}

fn print_output(output: String, may_words_per_line: Option<usize>) {
    if let Some(words_per_line) = may_words_per_line {
        let with_new_lines = new_lines::ensure_new_lines_by(output, words_per_line);
        println!("{}", with_new_lines);
    } else {
        println!("{}", output);
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
