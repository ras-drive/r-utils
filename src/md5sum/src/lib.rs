use clap::ArgMatches;
use md5::{compute, Digest};
use std::{io::{BufRead, Read}, path::Path, fs::{File, self}, collections::HashMap};

pub fn run(matches: ArgMatches) -> anyhow::Result<()> {
    if matches.is_present("check") {
        let mut failed = 0;
        let mut md5sums: HashMap<String, String> = HashMap::new();

        let sums_file = Path::new(matches.values_of("filename").unwrap().next().unwrap());
        let mut file = File::open(&sums_file).unwrap_or_else(|_| panic!("error while opening sums file: {}", sums_file.display()));
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => {
                for i in s.lines() {
                    let (value, key) = i.split_once(' ').unwrap();
                    md5sums.insert(String::from(key), String::from(value));
                }
            },
            Err(_) => {
                return Err(anyhow::Error::msg(""));
            },
        }

        let dir = fs::read_dir("./").unwrap();
        for path in dir {
            let filename = path.unwrap().file_name().into_string().unwrap();
            for (key, value) in &md5sums {
                if &filename == key {
                    let mut file = match File::open(&filename) {
                        Ok(file) => file,
                        Err(e) => return Err(anyhow::Error::msg(format!("error while opening file: {} {}", &filename, e))),
                    };

                    let mut s = String::new();
                    match file.read_to_string(&mut s) {
                        Ok(_) => {
                            // assert_eq!(&format!("{:?}", checksum(&s)), value);
                            if &format!("{:?}", checksum(&s)) == value {
                                if !matches.is_present("quiet") {
                                    println!("{}: OK", &filename);
                                }
                            } else {
                                println!("{}: FAILED", &filename);
                                failed += 1;
                            }
                        },
                        Err(e) => return Err(anyhow::Error::msg(format!("error while reading file: {} {}", &filename, e))),
                    }
                }
            }
        }
        if failed > 0 {
            println!("md5sum: WARNING: {} computed checksum  did NOT match", failed);
            return Err(anyhow::Error::msg("error, failed checksums while matching"));
        }

    } else {
        let mut hashmap = HashMap::new();
        if matches.is_present("filename") {
            let filenames = matches.values_of("filename").unwrap();
            for filename in filenames {
                    let path = String::from(filename);
                    let path = Path::new(&path);
                    let display = path.display();
                    if Path::new(path).is_file() {
                        let mut file = match File::open(&path) {
                            Err(why) => panic!("couldn't open {}: {}", display, why),
                            Ok(file) => file,
                        };
        
                        // Read the file contents into a string, returns `io::Result<usize>`
                        let mut s = String::new();
                        match file.read_to_string(&mut s) {
                            Err(why) => panic!("couldn't read {}: {}", display, why),
                            Ok(_) => {
                                hashmap.insert(display.to_string().clone(), format!("{:?}", checksum(&s).clone()));
                            },
                        }
                    } else if path.is_dir() {
                        return Err(anyhow::Error::msg("cannot take a directory as input"));
                    } else {
                        return Err(anyhow::Error::msg("error, input is not a file"));
                    }
                }
        } else {
            let input = std::io::stdin().lock().lines().next().unwrap().unwrap();
            hashmap.insert(String::from("-"), format!("{:?}", checksum(&input)));
        }

        for (key, value) in hashmap {
            if !matches.is_present("zero") {
                println!("{} {}", value, key);
            } else {
                print!("{} {}\u{0000}", value, key);
            }
        }
    }
        
    Ok(())
}

fn checksum(input: &String) -> Digest {
    compute(&input.as_bytes())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use clap::{Command, Arg};
    use rand::distributions::{Alphanumeric, DistString};

    use super::*;

    #[test]
    fn test_checksum() {
        let input = "hello world".to_string();
        let output = checksum(&input);
        const VERIFIED_SUM: &str = "5eb63bbbe01eeed093cb22bb8f5acdc3";
        assert_eq!(format!("{:x}", output), VERIFIED_SUM);
    }

    #[test]
    fn test_check() {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let matches = Command::new("test")
        .arg(Arg::new("filename")
            .required(false)
            .multiple_values(true)
            .help("With no FILE, or when FILE is -, read standard input."))
        .arg(Arg::new("check")
            .short('c')
            .long("check")
            .help("read checksums from the FILEs and check them"))
        .get_matches_from(vec!["test", "-c", &format!("{}.md5sum", &string)]);

        let mut md5 = File::create(Path::new(&format!("{}.md5sum", &string))).unwrap();
        md5.write_all(format!("5eb63bbbe01eeed093cb22bb8f5acdc3 {}", &format!("{}.txt", &string)).as_bytes()).unwrap();

        let mut file = File::create(Path::new(&format!("{}.txt", &string))).unwrap();
        file.write_all(b"hello world").unwrap();

        run(matches).unwrap();
        fs::remove_file(Path::new(&format!("{}.md5sum", &string))).unwrap();
        fs::remove_file(Path::new(&format!("{}.txt", &string))).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_should_panic() {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let matches = Command::new("test")
        .arg(Arg::new("filename")
            .required(false)
            .multiple_values(true)
            .help("With no FILE, or when FILE is -, read standard input."))
        .arg(Arg::new("check")
            .short('c')
            .long("check")
            .help("read checksums from the FILEs and check them"))
        .get_matches_from(vec!["test", "-c", &format!("{}.md5sum", &string)]);

        let mut md5 = File::create(Path::new(&format!("{}.md5sum", &string))).unwrap();
        md5.write_all(format!("5eb63bbbe01eeed093cb22bb8f5acdc3 {}", &format!("{}.txt", &string)).as_bytes()).unwrap();

        let mut file = File::create(Path::new(&format!("{}.txt", &string))).unwrap();
        file.write_all(b"NOT HELLO WORLD").unwrap();

        if run(matches).is_err() {
            fs::remove_file(Path::new(&format!("{}.md5sum", &string))).unwrap();
            fs::remove_file(Path::new(&format!("{}.txt", &string))).unwrap();
            panic!("test success!");
        }
    }
}
