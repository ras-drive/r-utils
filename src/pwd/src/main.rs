use std::env;
use clap::{Command, ArgMatches};

fn main() {
    let matches = Command::new("pwd")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("print name of current/working directory")
        .get_matches();

    run(matches);
}

fn run(matches: ArgMatches) {
    let binding = env::current_dir().unwrap();
    let working_directory = binding.to_str().unwrap();
    println!("{}", working_directory);
}