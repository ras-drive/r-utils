use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use clap::ArgMatches;
use crate::lib::arg_parser::Args;
use filetime::*;
use std::time::{SystemTime, UNIX_EPOCH};

mod arg_parser;

pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let args = Args::new(&matches)?;// .expect("error while parsing args");
    let exists = Path::new(args.filename).exists();
    let file = get_file(args.filename)?;

    // checks to see if program should exit with blank file created
    if !exists {
        std::process::exit(1);
    }

    set_file_mtime(args.filename, FileTime::now()).expect("TODO: panic message");

    Ok(())
}

pub fn get_file(filename: &str) -> Result<File, Box<dyn Error>>{
    match File::open(Path::new(filename)) {
        Ok(file) => {
            Ok(file)
        }
        Err(_) => {
            if !Path::new(filename).exists() {
                return Ok(File::create(filename).expect("error while creating file"));
            }
            Err(panic!("file not found"))
        }
    }
}