use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short)]
    application: String,

    #[arg()]
    file_identifier: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}", args.application);
    println!("File: {}", args.file_identifier);
}
