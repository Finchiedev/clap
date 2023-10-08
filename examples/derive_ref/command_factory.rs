use clap::{Command, CommandFactory, Parser};

#[derive(Parser, Debug)]
struct MyCommand {
    name: String,
}

fn main() {
    let command: Command = MyCommand::command();
    let matches = command.get_matches_from(["mycommand", "ferris"]);
    println!("Name: {}", matches.get_one::<String>("name").unwrap());
}
