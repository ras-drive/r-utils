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
        .arg(Arg::new("zero")
            .short('z')
            .long("zero")
            .help("end each output line with NUL, not newline, and disable file name escaping"))
        .arg(Arg::new("quiet")
            .long("quiet")
            .help("don't print OK for each successfully verified file"))
        .get_matches();

    run(matches).unwrap();
}
