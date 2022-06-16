mod lib;
mod syntax;
mod file_list;

use clap::{Arg, Command};

use crate::lib::lib::search;
use crate::syntax::syntax::{get_long, get_metadata};
use crate::file_list::file_list::FileList;

fn main() {
    println!("WARNING: THIS PROGRAM IS MAJORLY WIP!");
    let matches = Command::new("ls")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("")
        .arg(Arg::new("dir_name")
            .required(false))
        .arg(Arg::new("depth")
            .required(false))
        .arg(Arg::new("long")
            .required(false)
            .short('l')
            .long("long"))
        .get_matches();

    let mut dir_name = String::new();
    let mut depth = 1;

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

        let mut fl = FileList::new();
        fl.collect(search(Some(String::from(dir_name)), Some(depth)));

        if matches.is_present("long") {
            fl.print_long();
        } else {
            fl.print();
        }

    } else {
        let mut fl = FileList::new();
        fl.collect(search(None, Some(depth)));

        if matches.is_present("long") {
            fl.print_long();
        } else {
            fl.print();
        }
    }
}