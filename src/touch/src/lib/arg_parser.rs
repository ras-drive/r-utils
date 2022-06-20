use std::error::Error;
use std::io;
use clap::ArgMatches;

pub struct Args<'a> {
    pub filename: &'a str,
}

impl<'b> Args<'b> {
    pub fn new<'a>(matches: &'b ArgMatches) -> Result<Args<'b>, Box<dyn Error>> {
        if matches.is_present("filename") {
            let filename = matches.value_of("filename")
                .unwrap();
            return Ok(Args {
                filename
            })
        }
        Err("file not provided")?
    }
}