# lipsum-cli
version: 0.1.0

lipsum-cli is a small terminal application written in Rust language. It's used for generating pseudo-Latin lorem ipsum filler text in terminal.

> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat…

This application uses [lipsum](https://github.com/mgeisler/lipsum/) library to generate text.

It also has the ability to generate text from a file or stdin.

## INSTALLATION

```shell
cargo install lipsum-cli
```

## USAGE

```shell
ARGS:
    <SOURCE>    What mode to run the program in [default: liber-primus] [possible values: liber-
                primus, lorem-ipsum, custom]

OPTIONS:
    -f, --file <FILE>      File input for the custom source. If not specified, stdin is used
                           [default: ]
    -h, --help             Print help information
    -V, --version          Print version information
    -w, --words <WORDS>    Name of the person to greet [default: 0]
```

## EXAMPLES

This will generate a string using Liber Primus like
Grate Meminit et Praesentibus 
which should be suitable for use in a document title for section heading.

    $ lipsum-cli 
    Dolore sed in his Rebus Instructus

This will generate a string of 5 words from a file (lorem-ipsum.txt)
    
    $ lipsum-cli custom --file ./start-wars.txt
    Weapon, the DEATH STAR, an.

This will generate a string of 5 words from stdin
    
    $ cat start-wars.txt | ./target/debug/lipsum-cli custom
    The Empire’s sinister agents, Princess.

This will generate a string of 10 words from a file (lorem-ipsum.txt)
    
    $ lipsum-cli --file star-wars.txt --words 10
    Aboard her starship, custodian of the stolen plans that can.

This will generate a string of 10 words from stdin
    
    $ cat star-wars.txt | lipsum-cli custom --words 10
    The stolen plans that can save her people and restore.

This will generate a string of 10 words using the lorem ipsum example filler text beginning with “Lorem ipsum dolor sit amet”
    
    $ lipsum-cli lorem-ipsum --words 10
    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.

## LICENSE
    
This project is licensed under the MIT license.

## Release History

This is a changelog describing the most important changes per release.

### Version 0.1.0 — Feb 5th, 2022

- Initial release