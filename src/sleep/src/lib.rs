use clap::ArgMatches;
use regex::Regex;
use std::{thread, time};

pub fn get_duration(matches: ArgMatches) -> u64 {
    let re = Regex::new(r"[0-9]").unwrap();
    let number = matches.value_of("NUMBER").unwrap();
    if number.contains('s') {
        let caps = re.captures(number).unwrap();
        (caps.get(0).unwrap().as_str().parse::<u64>().unwrap()) * 1000
    } else if number.contains('m') {
        let caps = re.captures(number).unwrap();
        ((caps.get(0).unwrap().as_str().parse::<u64>().unwrap()) * 1000) * 60
    } else if number.contains('h') {
        let caps = re.captures(number).unwrap();
        ((caps.get(0).unwrap().as_str().parse::<u64>().unwrap()) * 1000) * 3600
    } else if number.contains('d') {
        let caps = re.captures(number).unwrap();
        (((caps.get(0).unwrap().as_str().parse::<u64>().unwrap()) * 1000) * 3600) * 24
    } else {
        let caps = re.captures(number).unwrap();
        (caps.get(0).unwrap().as_str().parse::<u64>().unwrap()) * 1000
    }
}

pub fn sleep(miliseconds: u64) {
    let duration_from = time::Duration::from_millis(miliseconds);
    let now = time::Instant::now();

    thread::sleep(duration_from);

    assert!(now.elapsed() >= duration_from);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, Command};

    #[test]
    fn one_second() {
        let matches = Command::new("test")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("delay for a specified amount of time")
        .arg(Arg::new("NUMBER")
            .required(true)
            .takes_value(true)
            .help("Pause  for NUMBER seconds.  SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an integer.  Given two or more arguments, pause for the amount of time specified by the sum of their values."))
        .get_matches_from(vec!["test", "1s"]);

        assert_eq!(1000, get_duration(matches));
    }

    #[test]
    fn one_minute() {
        let matches = Command::new("test")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("delay for a specified amount of time")
        .arg(Arg::new("NUMBER")
            .required(true)
            .takes_value(true)
            .help("Pause  for NUMBER seconds.  SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an integer.  Given two or more arguments, pause for the amount of time specified by the sum of their values."))
        .get_matches_from(vec!["test", "1m"]);

        assert_eq!(60_000, get_duration(matches));
    }

    #[test]
    fn one_hour() {
        let matches = Command::new("test")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("delay for a specified amount of time")
        .arg(Arg::new("NUMBER")
            .required(true)
            .takes_value(true)
            .help("Pause  for NUMBER seconds.  SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an integer.  Given two or more arguments, pause for the amount of time specified by the sum of their values."))
        .get_matches_from(vec!["test", "1h"]);

        assert_eq!(3_600_000, get_duration(matches));
    }

    #[test]
    fn one_day() {
        let matches = Command::new("test")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("delay for a specified amount of time")
        .arg(Arg::new("NUMBER")
            .required(true)
            .takes_value(true)
            .help("Pause  for NUMBER seconds.  SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an integer.  Given two or more arguments, pause for the amount of time specified by the sum of their values."))
        .get_matches_from(vec!["test", "1d"]);

        assert_eq!(86_400_000, get_duration(matches));
    }
}
