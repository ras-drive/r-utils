use clap::{Command, Arg};
use std::path::Path;
use std::io;
use std::fs;

fn main() {
    let matches = Command::new("mkdir")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("make directories")
        .arg(Arg::new("dir_name")
            .required(true)
            .takes_value(true)
            .help("")
            .multiple_values(true)
        )
        .get_matches();

    let dir_paths: Vec<&str> = matches.values_of("dir_name").unwrap().collect();

    check_dirs(&dir_paths).unwrap();
    make_dirs(&dir_paths).unwrap();

    println!("{:?}", dir_paths);
}


// split into two functions so that check_dirs() doesn't
// accidently make any paths before it can check that
// any of the paths will overwrite anything
fn check_dirs(dir_paths: &Vec<&str>) -> io::Result<()> {
    for path in dir_paths {
        if Path::new(path).exists() || Path::new(path).is_dir() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("file {} already exists", path)));
        }
    }

    Ok(())
}

fn make_dirs(dir_paths: &Vec<&str>) -> io::Result<()> {
    for path in dir_paths {
        fs::create_dir(path)?
    }

    Ok(())
}