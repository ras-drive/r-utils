use clap::{Arg, Command};
use r_utils::sleep::{get_duration, sleep};

fn main() {
    let matches = Command::new("sleep")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("delay for a specified amount of time")
        .arg(Arg::new("NUMBER")
            .required(true)
            .takes_value(true)
            .help("Pause  for NUMBER seconds.  SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an integer.  Given two or more arguments, pause for the amount of time specified by the sum of their values."))
        .get_matches();

    sleep(get_duration(matches));
}
