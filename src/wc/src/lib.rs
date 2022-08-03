use std::{fs, io, io::Error};
use std::io::BufRead;
use clap::{Arg, Command};

use regex::Regex;

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

pub fn run() {
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
    pub fn read_file(path: &str) -> Result<String, Error> {
        fs::read_to_string(path)
    }

    pub fn read_line_count(path: &str) -> usize {
        read_file(path).unwrap().lines().count()
    }

    pub fn read_word_count(path: &str) -> usize {
        let re = Regex::new("").unwrap();
        let mut count = 0;
        for _ in read_file(path).unwrap().split(" ") {
            if re.is_match("[a-zA-Z0-9]+([-_][a-zA-Z0-9]+)*") {
                count += 1
            }
        }
        count
    }

    pub fn read_char_count(path: &str) -> usize {
        read_file(path).unwrap().len()
    }

    pub fn read_byte_count(path: &str) -> usize {
        read_file(path).unwrap().as_bytes().to_vec().len()
    }


#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn test_read() {
        let mut file_ref = std::fs::File::create("test1.txt").expect("test: #1(test_read), failed on file write");
        file_ref
            .write_all("test\n".as_bytes())
            .expect("test: #1(test_read), failed on file write");

        let contents = match read_file("test1.txt") {
            Ok(data) => data,
            Err(_) => {
                println!("error: file not found");
                std::process::exit(1);
            }
        };

        assert_eq!("test\n", contents);
        std::fs::remove_file("test1.txt").unwrap();
    }

    #[test]
    fn read_byte_test() {
        let mut file_ref = std::fs::File::create("test2.txt")
            .expect("test: #2(read_byte_test), failed on file write");
        file_ref
            .write_all("test\n".as_bytes())
            .expect("test: #2(read_byte_test), failed on file write");

        assert_eq!(5, read_byte_count("test2.txt"));
        std::fs::remove_file("test2.txt").unwrap();
    }

    #[test]
    fn read_line_test() {
        let mut file_ref = std::fs::File::create("test3.txt")
            .expect("test: #3(read_line_test), failed on file write");
        file_ref
            .write_all("test\n test test\n\n\n  test test".as_bytes())
            .expect("test: #3(read_line_test), failed on file write");

        assert_eq!(5, read_line_count("test3.txt"));
        std::fs::remove_file("test3.txt").unwrap();
    }

    #[test]
    fn read_word_test() {
        let mut file_ref = std::fs::File::create("test4.txt")
            .expect("test: #4(read_word_test), failed on file write");
        file_ref
            .write_all("test testies test".as_bytes())
            .expect("test: #4(read_word_test), failed on file write");

        assert_eq!(3, read_word_count("test4.txt"));
        std::fs::remove_file("test4.txt").unwrap();
    }

    #[test]
    fn read_char_test() {
        let mut file_ref = std::fs::File::create("test5.txt")
            .expect("test: #5(read_char_test), failed on file write");
        file_ref
            .write_all("test test testing".as_bytes())
            .expect("test: #5(read_char_test), failed on file write");

        assert_eq!(17, read_char_count("test5.txt"));
        std::fs::remove_file("test5.txt").unwrap();
    }
}
