use std::{thread, time};

use clap::{Command, Arg, ArgMatches};

fn main() {
    let matches = Command::new("sleep")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("delay for a specified amount of time")
        .arg(Arg::new("NUMBER")
            .required(true)
            .takes_value(true)
            .help("Pause  for NUMBER seconds.  SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an integer.  Given two or more arguments, pause for the amount of time specified by the sum of their values."))
        .get_matches();

        sleep(get_duration(matches));
}

fn get_duration(matches: ArgMatches) -> u64 {
    let miliseconds;
    let number = matches.value_of("NUMBER").unwrap();
    if number.contains('s') {
        miliseconds = number.split('s').next().unwrap().parse::<u64>().unwrap() * 1000;
        miliseconds
    } else if number.contains('m') {
        miliseconds = (number.split('m').next().unwrap().parse::<u64>().unwrap() * 1000) * 3600;
        miliseconds
    } else if number.contains('d') {
        miliseconds = ((number.split('d').next().unwrap().parse::<u64>().unwrap() * 1000) * 3600) * 24;
        miliseconds
    } else {
        miliseconds = number.parse::<u64>().unwrap() * 1000;
        miliseconds
    }
}

fn sleep(miliseconds: u64) {
    let duration_from = time::Duration::from_millis(miliseconds);
    let now = time::Instant::now();

    thread::sleep(duration_from);

    assert!(now.elapsed() >= duration_from);
}