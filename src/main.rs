extern crate core;

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

enum Input {
    Stdin,
    File,
    LiberPrimus,
    LoremIpsum,
}

fn main() {
    let args = Args::parse();

    let input_source: Input;
    let file = args.file.unwrap_or("".to_string());
    let source = args.text_source;

    if file != "" {
        input_source = Input::File;
    } else if atty::isnt(Stream::Stdin) {
        input_source = Input::Stdin;
    } else if source == TextSource::LoremIpsum {
        input_source = Input::LoremIpsum;
    } else if source == TextSource::LiberPrimus {
        input_source = Input::LiberPrimus;
    } else {
        panic!("Invalid source");
    }

    let words = args.words.unwrap_or_else(|| {
        if matches!(input_source, Input::LiberPrimus) { 0 } else { 5 }
    });

    match input_source {
        Input::Stdin => {
            let input = utils::read_from_stdin();
            println!("{}", custom::run(&*input, words));
        }
        Input::File => {
            let input = utils::read_from_file(&file);
            println!("{}", custom::run(&*input, words));
        }
        Input::LoremIpsum => {
            println!("{}", lorem_ipsum::run(words));
        }
        Input::LiberPrimus => {
            println!("{}", liber_primus::run(words));
        }
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}