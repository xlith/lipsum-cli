# lipsum-cli

![](https://img.shields.io/crates/v/lipsum-cli?style=flat-square)
![](https://img.shields.io/badge/license-MIT-blue?style=flat-square)

lipsum-cli is a small terminal application written in Rust language. It's used for generating pseudo-Latin lorem ipsum filler text in terminal.

> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat…

This application uses [lipsum](https://github.com/mgeisler/lipsum/) library to generate text.

It also has the ability to generate text from a file or stdin.

## INSTALLATION

### Using Cargo
```shell
cargo install lipsum-cli
```

### Using Homebrew
```shell
brew tap xlith/lipsum-cli
brew install lipsum-cli
```

## USAGE

```
lipsum-cli [OPTIONS]

OPTIONS:
    -f, --file <FILE>                  File input for the custom source. This has priority to stdin.
                                       If not specified, stdin is used
    -h, --help                         Print help information
    -t, --text-source <TEXT_SOURCE>    Text source to choose from. Ignored if stdin or `-f` is used
                                       [default: liber-primus] [possible values: liber-primus,
                                       lorem-ipsum]
    -V, --version                      Print version information
    -w, --words <WORDS>                Count of words to generate. Default is 5 if text source is
                                       not liber-primus
    -l, --words-per-line               Count of words after a new line is inserted. By default deactivated
                                       Zero is the same as deactivated
                                       No new line will come after the last word

```

## EXAMPLES

This will generate a string using Liber Primus like
Grate Meminit et Praesentibus 
which should be suitable for use in a document title for section heading.

    $ lipsum-cli 
    Dolore sed in his Rebus Instructus

This will generate a string of 5 words from a file (lorem-ipsum.txt)
    
    $ lipsum-cli --file ./start-wars.txt
    Weapon, the DEATH STAR, an.

This will generate a string of 5 words from stdin
    
    $ cat start-wars.txt | lipsum-cli
    The Empire’s sinister agents, Princess.

This will generate a string of 10 words from a file (lorem-ipsum.txt)
    
    $ lipsum-cli --file star-wars.txt --words 10
    Aboard her starship, custodian of the stolen plans that can.

This will generate a string of 10 words from stdin
    
    $ cat star-wars.txt | lipsum-cli --words 10
    The stolen plans that can save her people and restore.

This will generate a string of 10 words using the lorem ipsum example filler text beginning with “Lorem ipsum dolor sit amet”
    
    $ lipsum-cli --text-source lorem-ipsum --words 10
    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.

This will generate a string of 20 words with a new line after 4th word.

    $ lipsum-cli --words 30 --words-per-line 4
    Multavit. -- Si sine
    causa, nollem me ab
    eo dissentiunt, sed certe
    non probes, eum quem
    ego arbitror unum vidisse
    verum maximisque erroribus animos
    hominum liberavisse et omnia
    tradidisse, quae.

## CONTRIBUTING
    
This is my first Rust project, so I'm sure there are some mistakes. If you have any questions, comments or ideas, please feel free to submit a pull request or open an issue.

### Contribution Guidelines:

- Fork the repository on GitHub
- Create a new branch
- Commit your changes
- Push your branch to GitHub
- Open a pull request

## LICENSE
    
This project is licensed under the [MIT license](LICENSE). Contributions will be accepted under the same license.

## Release History

This is a changelog describing the most important changes per release.

### Version 0.3.0 — 02.03.2023

- Add a new option (words-per-line)
- Fix readme typo

### Version 0.2.2 — 24.02.2023

- update dependencies
- update creates.io metadata and readme

### Version 0.2.1 — 21.02.2023

- Small refactor for more idiomatic Rust
- Fix bug in which `--words` input was overridden

### Version 0.2.0 — 30.09.2022

- Refactor code for better readability
- Fix bug when stdin input length is 0
- Update dependencies

### Version 0.2.0 — 19.02.2022

- Completely rewritten and redesigned the application to be more modular and easier to use.
- Fix lots of text and typos.
- Added unit tests.

### Version 0.1.0 — 20.02.2022

- Initial release
