use clap::ArgMatches;
use md5::{compute, Digest};
use std::{io::{BufRead, Read}, path::Path, fs::File};

pub fn run(matches: ArgMatches) -> anyhow::Result<()> {
    let filename = matches.value_of("filename").unwrap_or("-");
    let mut input: String = match matches.is_present("filename") {
        true => {
            let path = matches.value_of("filename").unwrap().to_string();
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
                        s
                    },
                }
            } else if path.is_dir() {
                return Err(anyhow::Error::msg("cannot take a directory as input"));
            } else {
                return Err(anyhow::Error::msg("error, input is not a file"));
            }


        },
        false => std::io::stdin().lock().lines().next().unwrap().unwrap(),
    };

    // TODO: check for -n flag to see if it should forego the newline
    if !matches.is_present("filename") {
        input.push('\n');
    }
    println!("{:?} {}", checksum(&input), filename);

    Ok(())
}

fn checksum(input: &String) -> Digest {
    compute(&input.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let input = "hello world".to_string();
        let output = checksum(&input);
        assert_eq!(format!("{:x}", output), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }
}
