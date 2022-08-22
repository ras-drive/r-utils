use std::fs;
use std::path::Path;
use clap::{ArgMatches, Arg, Command};

#[allow(clippy::needless_lifetimes)]
pub fn check_files<'a>(matches: &'a ArgMatches, paths: &'a Vec<&'a str>) -> Vec<&'a Path> {
    let mut valid_paths = vec![];

            for i in paths {
                let path = Path::new(i);
                if !path.exists() {
                    panic!("file {:?} doesn't exist", path);
                } else if path.is_dir() && !matches.is_present("recursive") {
                    panic!(
                        "{:?} is a directory but -r modifier was not specified",
                        path
                    );
                } else {
                    valid_paths.push(path)
                }
            }
        
    valid_paths
}

pub fn remove_files<'a>(matches: &'a ArgMatches, valid_paths: Vec<&'a Path>) -> Result<(), Box<dyn std::error::Error>> {
    for path in valid_paths {
        if path.is_dir() && matches.is_present("recursive") {
            fs::remove_dir(path)
                .unwrap_or_else(|_| panic!("error while removing directory {:?}", path));
        } else {
            fs::remove_file(path)
                .unwrap_or_else(|_| panic!("error while removing file {:?}", path));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rand::distributions::{Alphanumeric, DistString};

    use super::*;

    #[test]
    fn test_file() {
        // test Clap App 
        let matches = Command::new("test").get_matches();
        
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        fs::File::create(&string).unwrap_or_else(|_| panic!("Error while creating test file {}", &string));

        let path_vec = vec![string.as_str()];

        let valid_files = check_files(&matches, &path_vec);
        remove_files(&matches, valid_files).expect("error while removing test files");

        if Path::new(&string).exists() {
            fs::remove_file(&string).expect("error while handling last error");
            panic!("file still exists");
        }
    }

    #[test]
    fn test_dir() {
        // test Clap App with necessary flags
        let matches = Command::new("test")
        .arg(Arg::new("recursive")
            .short('r'))
        .get_matches_from(vec!["test", "-r"]);

        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        fs::create_dir(&string).unwrap_or_else(|_| panic!("Error while creating test dir {}", &string));

        let path_vec = vec![string.as_str()];

        let valid_files = check_files(&matches, &path_vec);
        remove_files(&matches, valid_files).expect("error while removing test dir");

        if Path::new(&string).exists() {
            fs::remove_dir(&string).expect("error while handling last error");
            panic!("dir still exists");
        }
    }
}