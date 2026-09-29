use crate::cli::Cli;
use clap::Parser;
use std::error::Error;

mod cli;

type CliResult = Result<(), Box<dyn Error>>;

fn main() -> CliResult {
    let cli = Cli::parse();

    let mut text = cli.text.join(" ");

    text.push_str(if cli.omit_newline { "" } else { "\n" });

    print!("{}", text);

    Ok(())
}
