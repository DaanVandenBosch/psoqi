# PSOQI

## Details and Usage

Psoqi is a command line program for extracting the quest name, short description and enemy counts from Phantasy Star Online quest files. It also tries to determine the quest's episode by looking at the kind of enemies it contains and in which areas those enemies are located, it defaults to episode I. It's written in Rust and should run anywhere Rust runs.

    USAGE:
        psoqi [FLAGS] <INPUT>...

    FLAGS:
        -c, --csv        Output information in CSV format
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <INPUT>...    Files and/or directories to process

## Limitations

* Only .qst files are supported
* Parsing errors are not reported

## To Do (In Order of Priority)

* Better error reporting
* Detect corrupted files
* Extract more information
* Support more formats (i.e. compressed and uncompressed .dat and .bin files)

## Building

Install Rust and run a debug build with `cargo run -- <PSOQI ARGS>` or build a release version with `cargo build --release`.