use clap::{Arg, Command};

use md5sum::run;

fn main() {
    let matches = Command::new("md5sum")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("print or check md5 checksums")
        .arg(Arg::new("filename")
            .required(false)
            .multiple_values(true)
            .help("With no FILE, or when FILE is -, read standard input."))
        .arg(Arg::new("check")
            .short('c')
            .long("check")
            .help("read checksums from the FILEs and check them"))
        .get_matches();

    run(matches).unwrap();
}
