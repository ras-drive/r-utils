use std::path::Path;
use std::fs;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("rm")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("remove files or directories")
        .arg(
            Arg::new("recursive")
                .short('r'))
        .arg(
            Arg::new("paths")
                .required(true)
                .takes_value(true)
                .help("paths of file(s)/directory(ies) to remove")
                .multiple_values(true))
        .get_matches();

        match matches.values_of("paths") {
            Some(paths) => {
                let mut valid_paths = vec![];
                for i in paths {
                    let path = Path::new(i);
                    if !path.exists() {
                        panic!("file {:?} doesn't exist", path);
                    } else if path.is_dir() && !matches.is_present("recursive") {
                        panic!("{:?} is a directory but -r modifier was not specified", path);
                    } else {
                        valid_paths.push(path)
                    }
                }
                for path in valid_paths {
                    if path.is_dir() && matches.is_present("recursive") {
                        fs::remove_dir(path).unwrap_or_else(|_| panic!("error while removing directory {:?}", path));
                    } else {
                        fs::remove_file(path).unwrap_or_else(|_| panic!("error while removing file {:?}", path));
                    }
                }
            },
            None => panic!("this shouldn't happen, clap should catch that no paths were specified"),
        }
}
