use clap::{ArgMatches, Command, Arg};
use std::path::Path;
use std::io;
use std::fs;

fn main() {
    let matches = Command::new("mkdir")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("Create the directory(ies), if they do not already exist.")
        .arg(Arg::new("directories")
            .required(true)
            .takes_value(true)
            .help("path of directory(ies) to create")
            .multiple_values(true))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .required(false)
            .takes_value(false)
            .help("print a message for each created directory"))
        .get_matches();

    let dir_paths: Vec<&str> = matches.values_of("directories").unwrap().collect();

    check_dirs(&dir_paths).expect("error while checking to see if supplied dirs exist");
    make_dirs(&matches, &dir_paths).expect("error while creating supplied dirs");
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

fn make_dirs(matches: &ArgMatches, dir_paths: &Vec<&str>) -> io::Result<()> {
    for path in dir_paths {
        fs::create_dir(path)?;
        if matches.is_present("verbose") {
            println!("mkdir: created directory {}", path);
        }
    }

    Ok(())
}
