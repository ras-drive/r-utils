use clap::ArgMatches;
use std::{fs, io, path::Path};

// split into two functions so that check_dirs() doesn't
// accidently make any paths before it can check that
// any of the paths will overwrite anything

pub fn check_dirs(dir_paths: &Vec<&str>) -> io::Result<()> {
    for path in dir_paths {
        if Path::new(path).exists() || Path::new(path).is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("file {} already exists", path),
            ));
        }
    }

    Ok(())
}

pub fn make_dirs(matches: &ArgMatches, dir_paths: &Vec<&str>) -> io::Result<()> {
    for path in dir_paths {
        fs::create_dir(path)?;
        if matches.is_present("verbose") {
            println!("mkdir: created directory {}", path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, Command};
    use rand::distributions::{Alphanumeric, DistString};

    #[test]
    fn test_check() {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        check_dirs(&vec![&string]).unwrap()
    }

    #[test]
    fn test_make() {
        let args = Command::new("").arg(Arg::new("verbose")).get_matches();
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        check_dirs(&vec![&string]).unwrap();
        make_dirs(&args, &vec![&string]).unwrap();

        fs::remove_dir_all(&string).unwrap();
    }
}
