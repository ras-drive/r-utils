use std::{
    io::{self, BufRead, Read},
    path::Path,
};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("nl")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("number lines of files")
        .arg(Arg::new("filename").takes_value(true))
        .get_matches();

    if matches.is_present("filename") {
        let path = Path::new(matches.value_of("filename").unwrap());
        let mut file = match std::fs::File::open(&path) {
            Err(err) => panic!("couldn't open {}: {}", path.display(), err),
            Ok(file) => file,
        };

        let mut s = String::new();
        #[allow(clippy::single_match)]
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
            Ok(_) => (),
        }
        let mut lines = 0;
        for line in s.lines() {
            lines += 1;
            println!("      {}  {}", lines, line)
        }
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
