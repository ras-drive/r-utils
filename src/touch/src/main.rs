use std::io;
use clap::{Arg, Command};

fn main() -> Result<(), io::Error> {
    println!("This project is highly WIP!");
    let matches = Command::new("touch")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("changes filetime to now, if file doesn't exist create an empty file with same name")
        .arg(Arg::new("filename"))
        .get_matches();

    run()
}

fn run() -> Result<(), io::Error> {
    Ok(())
}