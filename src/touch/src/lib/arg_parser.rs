use clap::ArgMatches;
use std::error::Error;

pub struct Args<'a> {
    pub filename: &'a str,
    pub create: bool,
    pub date: &'a str,
    pub modify: bool,
    pub access: bool,
}

impl<'b> Args<'b> {
    pub fn new<'a>(matches: &'b ArgMatches) -> Result<Args<'b>, Box<dyn Error>> {
        let mut create = true;
        let mut date = "";
        let mut modify = false;
        let mut access = false;

        if matches.is_present("create") {
            create = false;
        }

        if matches.is_present("date") {
            date = matches.value_of("date").unwrap();
        }

        if matches.is_present("modify") {
            modify = true;
        }

        if matches.is_present("access") {
            access = true;
        }

        if matches.is_present("filename") {
            let filename = matches.value_of("filename").unwrap();
            return Ok(Args {
                filename,
                create,
                date,
                modify,
                access,
            });
        }
        Err("file not provided")?
    }
}
