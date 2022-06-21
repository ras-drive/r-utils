use std::error::Error;
use std::io;
use clap::ArgMatches;

pub struct Args<'a> {
    pub filename: &'a str,
    pub create: bool,
    pub date: &'a str
}

impl<'b> Args<'b> {
    pub fn new<'a>(matches: &'b ArgMatches) -> Result<Args<'b>, Box<dyn Error>> {
        let mut create = true;
        let mut date = "";

        if matches.is_present("create") {
            create = false;
        }

        if matches.is_present("date") {
            date = matches.value_of("date").unwrap();
        }

        if matches.is_present("filename") {
            let filename = matches.value_of("filename")
                .unwrap();
            return Ok(Args {
                filename,
                create,
                date
            })
        }
        Err("file not provided")?
    }
}