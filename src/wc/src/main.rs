extern crate core;

use std::io::{self, BufRead};
use std::string::String;

use clap::{Arg, Command};

use wc::lib::{read_byte_count, read_char_count, read_line_count, read_word_count};

mod lib;

fn main() {
    let matches = Command::new("wc")
        .version("0.01")
        .author("Sarah Petkovic")
        .about("A simple wc clone in Rust")
        .arg(Arg::new("filename"))
        .arg(Arg::new("byte_count")
                 .short('c')
                 .long("bytes")
                 .required(false),
        )
        .arg(Arg::new("char_count")
                 .short('m')
                 .long("chars")
                 .required(false),
        )
        .arg(Arg::new("word_count")
                 .short('w')
                 .long("words")
                 .required(false),
        )
        .arg(Arg::new("line_count")
            .short('l')
            .long("lines")
            .required(false)
        )
        .get_matches();

    let filename: String;

    if matches.is_present("filename") {
        let mut output = String::new();

        filename = matches
            .value_of("filename")
            .expect("error while reading provided file name")
            .parse()
            .unwrap();

        if matches.is_present("byte_count") {
            output.push_str(format!("{} ", read_byte_count(filename.as_str()).to_string()).as_str());
        } else if matches.is_present("char_count") {
            output.push_str(&*format!("{} ", read_char_count(filename.as_str())));
        } else if matches.is_present("word_count") {
            output.push_str(&*format!("{} ", read_word_count(filename.as_str())));
        } else if matches.is_present("line_count") {
            output.push_str(&*format!("{} ", read_line_count(filename.as_str())));
        } else {
            // default output
            output = format!(
                "{} {} {} ",
                read_line_count(filename.as_str()),
                read_word_count(filename.as_str()),
                read_byte_count(filename.as_str()).to_string().as_str()
            );
        }

        output.push_str(&*filename);
        println!("{}", output);
    } else {
        loop {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                println!("{}", line.unwrap());
            }
        }
    }
}
