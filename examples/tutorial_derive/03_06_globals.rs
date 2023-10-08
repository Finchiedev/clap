use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(global = true, long = "name")]
    name: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Greet,
}

fn main() {
    let cli = Cli::parse();

    println!("Name: {:?}", cli.name);
}
