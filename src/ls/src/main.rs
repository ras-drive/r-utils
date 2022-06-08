mod lib;

use std::fmt::format;
use std::path::{Path, PathBuf};
use clap::{Arg, Command};
use ls::lib::search;


fn main() {
    let matches = Command::new("ls")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("")
        .arg(Arg::new("dir_name")
            .required(false))
        .arg(Arg::new("depth"))
        .get_matches();

    let mut dir_name: String = String::new();
    let mut depth: usize = 1;

    if matches.is_present("depth") {
        depth = matches
            .value_of("depth")
            .expect("")
            .parse()
            .unwrap()
    }

    if matches.is_present("dir_name") {
        dir_name = matches
            .value_of("dir_name")
            .expect("error while reading provided directory name")
            .parse()
            .unwrap();

        search(Some(String::from(dir_name)), Some(depth));
    } else {
        search(None, None);
    }
}