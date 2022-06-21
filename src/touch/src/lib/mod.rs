use std::error::Error;
use std::fs::File;
use std::path::Path;
use chrono::{NaiveDate, NaiveDateTime};
use clap::ArgMatches;
use crate::lib::arg_parser::Args;
use filetime::*;

mod arg_parser;

#[allow(unused_variables)]
pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let args = Args::new(&matches)?;// .expect("error while parsing args");
    let date = "";

    if Path::exists(Path::new(args.filename)) {
        if matches.is_present("date") {
            let date = parse_date(&args)?;

            set_file_atime(args.filename, FileTime::from_unix_time(date.timestamp(), 0)).expect("TODO: panic message");
            set_file_mtime(args.filename, FileTime::from_unix_time(date.timestamp(), 0)).expect("TODO: panic message");
        }

        if matches.is_present("modify") {
            set_file_mtime(args.filename, FileTime::now()).expect("TODO: panic message");
        }

        if matches.is_present("access") {
            set_file_atime(args.filename, FileTime::now()).expect("TODO: panic message");
        }
    } else {
        if args.create {
            File::create(args.filename).expect("TODO: panic message");
        } else {
            std::process::exit(1);
        }
    }

    Ok(())
}

fn parse_date(args: &Args) -> Result<NaiveDateTime, Box<dyn Error>> {
    let date_time;
    let date = args.date.split(" ").nth(0).unwrap();
    let time = args.date.split(" ").nth(1).unwrap();

    if date.contains("/") {
        date_time = NaiveDate::from_ymd( date.split("/").nth(0).expect("error parsing provided date").parse().expect("error parsing provided date"),
                                         date.split("/").nth(1).expect("error parsing provided date").parse().expect("error parsing provided date"),
                                         date.split("/").nth(2).expect("error parsing provided date").parse().expect("error parsing provided date"))
            .and_hms(time.split(":").nth(0).expect("error parsing provided date").parse().expect("error parsing provided date"),
                     time.split(":").nth(1).expect("error parsing provided date").parse().expect("error parsing provided date"),
                     time.split(":").nth(2).expect("error parsing provided date").parse().expect("error parsing provided date"));
        return Ok(date_time)
    } else if date.contains("-") {
        date_time = NaiveDate::from_ymd( date.split("-").nth(0).expect("error parsing provided date").parse().expect("error parsing provided date"),
                                         date.split("-").nth(1).expect("error parsing provided date").parse().expect("error parsing provided date"),
                                         date.split("-").nth(2).expect("error parsing provided date").parse().expect("error parsing provided date"))
            .and_hms(time.split(":").nth(0).expect("error parsing provided date").parse().expect("error parsing provided date"),
                     time.split(":").nth(1).expect("error parsing provided date").parse().expect("error parsing provided date"),
                     time.split(":").nth(2).expect("error parsing provided date").parse().expect("error parsing provided date"));
        return Ok(date_time)
    } else {
        Err("invalid date format")?
    }
}