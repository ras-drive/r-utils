use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use clap::ArgMatches;
use crate::lib::arg_parser::Args;

mod arg_parser;

pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let args = Args::new(&matches)?;// .expect("error while parsing args");
    let file = get_file(args.filename)?;
    println!("Success");
    Ok(())
}

pub fn get_file(filename: &str) -> Result<File, Box<dyn Error>>{
    match File::open(Path::new(filename)) {
        Ok(file) => {
            Ok(file)
        }
        Err(_) => {
            panic!("file not found")
        }
    }
}