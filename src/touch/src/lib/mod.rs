use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::Prefix::UNC;
use std::time::SystemTime;
use chrono::{NaiveDate, NaiveDateTime};
use clap::ArgMatches;
use crate::lib::arg_parser::Args;
use filetime::*;

mod arg_parser;

pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let args = Args::new(&matches)?;// .expect("error while parsing args");
    let file = get_file(args.filename)?;
    let mut date = "";

    if matches.is_present("date") {
        let date = parse_date(&args)?;

        set_file_atime(args.filename, FileTime::from_unix_time(date.timestamp(), 0)).expect("TODO: panic message");
        set_file_mtime(args.filename, FileTime::from_unix_time(date.timestamp(), 0)).expect("TODO: panic message");

        std::process::exit(1);
    }


    // create arg check
    if args.create {
        set_file_atime(args.filename, FileTime::now()).expect("TODO: panic message");
        set_file_mtime(args.filename, FileTime::now()).expect("TODO: panic message");
    }
    // println!("{}", date.unwrap());
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

fn parse_date(args: &Args) -> Result<NaiveDateTime, Box<dyn Error>> {
    let mut date_time;
    let mut date = args.date.split(" ").nth(0).unwrap();
    let mut time = args.date.split(" ").nth(1).unwrap();

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