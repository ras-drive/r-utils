use clap::ArgMatches;
use md5::{compute, Digest};
use std::io::BufRead;

pub fn run(matches: ArgMatches) {
    let filename = matches.value_of("filename").unwrap_or("-");
    let input = match matches.is_present("filename") {
        true => matches.value_of("filename").unwrap().to_string(),
        false => std::io::stdin().lock().lines().next().unwrap().unwrap(),
    };
    println!("{:?} {}", checksum(&input), filename);
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
