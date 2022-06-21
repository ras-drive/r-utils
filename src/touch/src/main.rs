mod lib;

use std::any::Any;
use std::error::Error;
use std::io;
use clap::{Arg, ArgMatches, Command};
use lib::run;

fn main() -> Result<(), io::Error> {
    println!("This project is highly WIP!");
    let matches = Command::new("touch")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("changes filetime to now, if file doesn't exist create an empty file with same name")
        .arg(Arg::new("filename").required(true))
        .arg(Arg::new("create")
            .required(false)
            .short('c')
            .long("no-create"))
        .arg(Arg::new("date")
            .required(false)
            .short('d')
            .long("date")
            .takes_value(true))
        .get_matches();

    run(matches).expect("TODO: panic message");
    Ok(())
}

