use clap::ArgMatches;
use promptly::prompt_default;
use std::fs::{self, ReadDir};
use std::path::Path;

#[allow(clippy::needless_lifetimes)]
pub fn check_files<'a>(
    matches: &'a ArgMatches,
    paths: &'a Vec<&'a str>,
) -> anyhow::Result<Vec<&'a Path>> {
    let mut valid_paths = vec![];

    for i in paths {
        let path = Path::new(i);
        if !path.exists() {
            panic!("file {:?} doesn't exist", path);
        } else if path.is_dir() && !matches.is_present("recursive") {
            //"{:?} is a directory but -r modifier was not specified",
            return Err(anyhow::Error::msg(""));
        } else {
            valid_paths.push(path)
        }
    }

    Ok(valid_paths)
}

pub fn remove_files<'a>(matches: &'a ArgMatches, valid_paths: Vec<&'a Path>) -> anyhow::Result<()> {
    for path in valid_paths {
        if path.is_dir() && matches.is_present("recursive") {
            let files = fs::read_dir(path).unwrap_or_else(|_| {
                panic!(
                    "error while reading dir {}",
                    path.as_os_str().to_str().unwrap()
                )
            });
            if matches.is_present("interactive") {
                let prompt =
                    prompt_default(format!("remove dir {}?", path.to_str().unwrap()), false);
                if !prompt.unwrap() {
                    continue;
                }
            }
            recursive_search_remove(matches, files)?;
            fs::remove_dir(path)
                .unwrap_or_else(|_| panic!("error while removing directory {:?}", path));

            if matches.is_present("verbose") {
                println!("removed {}", path.to_str().unwrap())
            }
        } else {
            if matches.is_present("interactive") {
                let prompt =
                    prompt_default(format!("remove file {}?", path.to_str().unwrap()), false);
                if !prompt.unwrap() {
                    continue;
                }
            }
            fs::remove_file(path)
                .unwrap_or_else(|_| panic!("error while removing file {:?}", path));
            if matches.is_present("verbose") {
                println!("removed {}", path.to_str().unwrap())
            }
        }
    }

    Ok(())
}

fn recursive_search_remove(matches: &ArgMatches, read_dir: ReadDir) -> anyhow::Result<()> {
    for i in read_dir {
        match i {
            Ok(entry) => {
                if entry.path().is_dir() {
                    recursive_search_remove(
                        matches,
                        fs::read_dir(entry.path().to_str().unwrap()).unwrap(),
                    )?;
                    if matches.is_present("interactive") {
                        let prompt = prompt_default(
                            format!("remove file {}?", entry.file_name().to_str().unwrap()),
                            false,
                        );
                        if !prompt.unwrap() {
                            continue;
                        }
                    }
                    fs::remove_dir(entry.path().to_str().unwrap())?;
                } else if entry.path().is_file() {
                    if matches.is_present("interactive") {
                        let prompt = prompt_default(
                            format!("remove file {}?", entry.file_name().to_str().unwrap()),
                            false,
                        );
                        if !prompt.unwrap() {
                            continue;
                        }
                    }
                    fs::remove_file(entry.path())?;
                    if matches.is_present("verbose") {
                        println!("removed {}", entry.file_name().to_str().unwrap())
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir;

    use clap::{Arg, Command};
    use rand::distributions::{Alphanumeric, DistString};

    #[test]
    fn test_file() {
        // test Clap App
        let matches = Command::new("test")
            .arg(Arg::new("verbose"))
            .arg(Arg::new("interactive"))
            .get_matches();

        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        fs::File::create(&string)
            .unwrap_or_else(|_| panic!("Error while creating test file {}", &string));

        let path_vec = vec![string.as_str()];

        let valid_files = check_files(&matches, &path_vec).unwrap();
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
            .arg(Arg::new("verbose"))
            .arg(Arg::new("interactive"))
            .arg(Arg::new("recursive").short('r'))
            .get_matches_from(vec!["test", "-r"]);

        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        fs::create_dir(&string)
            .unwrap_or_else(|_| panic!("Error while creating test dir {}", &string));

        let path_vec = vec![string.as_str()];

        let valid_files = check_files(&matches, &path_vec).unwrap();
        remove_files(&matches, valid_files).expect("error while removing test dir");

        if Path::new(&string).exists() {
            fs::remove_dir(&string).expect("error while handling last error");
            panic!("dir still exists");
        }
    }

    #[test]
    #[should_panic]
    fn test_dir_no_flags() {
        // test Clap App
        let matches = Command::new("test")
            .arg(Arg::new("recursive"))
            .arg(Arg::new("interactive"))
            .arg(Arg::new("recursive"))
            .get_matches_from(vec!["test"]);

        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        fs::create_dir(&string)
            .unwrap_or_else(|_| panic!("Error while creating test dir {}", &string));

        let path_vec = vec![string.as_str()];

        match check_files(&matches, &path_vec) {
            Err(_) => {
                remove_dir(string).unwrap();
                panic!("success, test panics and deletes test dir")
            }
            Ok(_) => remove_dir(string).unwrap(), // test fails, still deletes test dir,
        }
    }
}
