use std::io::BufRead;
use clap::{Arg, ArgMatches, Command};
use md5::{Digest, compute};

fn main() {
    let matches = Command::new("md5sum")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("print or check md5 checksums")
        .arg(Arg::new("filename").required(false))
        .get_matches();

    run(matches);
}

fn run(matches: ArgMatches) {
    let filename = matches.value_of("filename").unwrap_or("-");
    let input;
    match matches.is_present("filename") {
        true => {
            input = matches.value_of("filename").unwrap().to_string();
        }
        false => {
            input = std::io::stdin().lock().lines().next().unwrap().unwrap();
        }
    }
    println!("{:?} {}", checksum(&input), filename);

}

fn checksum(input: &String) -> Digest {
    compute(&input.as_bytes())
}

mod tests {
    use super::*;
    #[test]
    fn test_checksum() {
        let input = "hello world".to_string();
        let output = checksum(&input);
        assert_eq!(format!("{:x}", output), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }
}