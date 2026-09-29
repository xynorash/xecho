use clap::Parser;

#[derive(Parser)]
#[command(
    name = "xecho",
    author = "Xynorash <nashtefison@gmail.com",
    about = "A Rust version of echo from GNU, developed by Xynorash",
    version = "0.1.0"
)]
pub(crate) struct Cli {
    /// Just what you want to output
    #[arg(required = true)]
    pub(crate) text: Vec<String>,

    /// Remove the trailing newline
    #[arg(short = 'n')]
    pub(crate) omit_newline: bool,
}
