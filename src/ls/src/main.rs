// #![feature(fmt_internals)]

mod lib;
mod ext_hash;
mod file;
mod syntax;

use std::any::Any;
use std::fmt::format;
use std::path::{Path, PathBuf};

use clap::{Arg, Command};

use crate::lib::lib::search;
use crate::ext_hash::ext_hash::{ExtHash};
use crate::file::file::File;

fn main() {
    println!("WARNING: THIS PROGRAM IS MAJORLY WIP!");
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

        let mut hash = ExtHash::new();

        let mut file = File::new(None);
        println!("{:?}", file.get_name());
    }
}