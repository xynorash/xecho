use clap::Command;

fn main() {
    Command::new("xecho")
        .author("Xynorash <nashtefison@gmail.com")
        .about("A Rust version of the echo command on GNU, built by Xynorash")
        .version("0.1.0")
        .get_matches();
}
