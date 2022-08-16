use std::io::{self, BufRead};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("nl")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("number lines of files")
        .arg(Arg::new("filename"))
        .get_matches();

    if matches.is_present("filename") {
    } else {
        let mut lines = 0;
        loop {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                lines += 1;
                println!("      {}  {}", lines, line.unwrap());
            }
        }
    }
}
