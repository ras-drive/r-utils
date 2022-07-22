extern crate core;

use std::io::{self, BufRead};
use std::string::String;

use clap::{Arg, Command};

use wc::lib::{read_byte_count, read_char_count, read_line_count, read_word_count};

mod lib;

///
/// ## Name
/// wc - print newline, word, and byte counts for each file
///
/// ## Description
/// Print  newline,  word, and byte counts for each FILE, and a total line if more than one FILE is
/// specified.  A word is a non-zero-length sequence of printable  characters  delimited  by  white
/// space.
///
/// ## Arguments
/// - -c, --bytes:
///     prints the byte count
/// - -m, --chars:
///     prints the character count
/// - -w, --words:
///     prints the word count
/// - -l, --lines:
///     prints the line count
///

fn main() {
    let matches = Command::new("wc")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("print newline, word, and byte counts for each file")
        .arg(Arg::new("filename"))
        .arg(Arg::new("byte_count")
            .short('c')
            .long("bytes")
            .help("prints the byte count")
            .required(false)
        )
        .arg(Arg::new("char_count")
                 .short('m')
                 .long("chars")
                 .help("prints the character count")
                 .required(false),
        )
        .arg(Arg::new("word_count")
                 .short('w')
                 .long("words")
                 .help("prints the word count")
                 .required(false),
        )
        .arg(Arg::new("line_count")
            .short('l')
            .long("lines")
            .help("prints the line count")
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
            output.push_str(&*format!("{} ", read_byte_count(filename.as_str())));
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
