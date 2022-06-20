mod lib;

use std::io;
use clap::{Arg, Command};
use lib::run;

fn main() -> Result<(), io::Error> {
    println!("This project is highly WIP!");
    let matches = Command::new("touch")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("changes filetime to now, if file doesn't exist create an empty file with same name")
        .arg(Arg::new("filename").required(true))
        .get_matches();

    run(matches).expect("TODO: panic message");
    Ok(())
}
