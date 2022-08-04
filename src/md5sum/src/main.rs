use clap::{Arg, Command};

use md5sum::run;

fn main() {
    let matches = Command::new("md5sum")
        .version("1.0")
        .author("Sarah Petkovic")
        .about("print or check md5 checksums")
        .arg(Arg::new("filename").required(false))
        .get_matches();

    run(matches);
}
