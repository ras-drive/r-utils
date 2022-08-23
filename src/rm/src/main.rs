#![feature(iter_collect_into)]

use clap::{Arg, Command};
use rm::{check_files, remove_files};

fn main() {
    let matches = Command::new("rm")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("remove files or directories")
        .arg(Arg::new("recursive").short('r'))
        .arg(Arg::new("verbose").short('v'))
        .arg(Arg::new("interactive").short('i'))
        .arg(
            Arg::new("paths")
                .required(true)
                .takes_value(true)
                .help("paths of file(s)/directory(ies) to remove")
                .multiple_values(true),
        )
        .get_matches();

    match matches.values_of("paths") {
        Some(values) => {
            let mut paths = vec![];
            let valid_paths = check_files(&matches, values.collect_into(&mut paths)).unwrap();
            remove_files(&matches, valid_paths).unwrap();
        }
        None => panic!("this shouldn't happen, clap should catch that no paths were specified"),
    }
}
