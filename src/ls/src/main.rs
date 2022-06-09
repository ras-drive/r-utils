mod lib;
mod syntax;
mod file_list;

use std::any::Any;
use std::borrow::Borrow;
use std::fmt::format;
use std::path::{Path, PathBuf};

use clap::{Arg, Command};

use crate::lib::lib::search;
use crate::syntax::syntax::get_long;

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

        let file_name = "test.txt";
        let meta = syntax::syntax::get_metadata(file_name);
        println!("{:?}", get_long(meta, file_name));
    }
}