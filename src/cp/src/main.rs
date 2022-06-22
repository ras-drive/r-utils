mod lib;

use clap::{Arg, Command};
use crate::lib::search;

fn main() {
    let matches = Command::new("cp")
        .version("0.1")
        .author("Sarah Petkovic")
        .about("")
        .arg(Arg::new("dir_name").required(true))
        .get_matches();

    for i in search(matches, false).unwrap() {
        println!("{}", i.path().canonicalize().unwrap().display())
    }
}
