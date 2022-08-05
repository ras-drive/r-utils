mod lib;

use clap::{Arg, Command};

use crate::lib::run;
use crate::lib::search;

fn main() {
    println!("WARNING: THIS PROGRAM IS MAJORLY WIP!");
    let matches = Command::new("ls")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("")
        .arg(Arg::new("dir_name").required(false))
        .arg(Arg::new("depth").required(false))
        .arg(Arg::new("long").required(false).short('l').long("long"))
        .get_matches();

    run(matches);
}
