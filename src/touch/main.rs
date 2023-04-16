use clap::{Arg, Command};
use r_utils::touch::run;
use std::io;

fn main() -> Result<(), io::Error> {
    let matches = Command::new("touch")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("changes filetime to now, if file doesn't exist create an empty file with same name")
        .arg(Arg::new("filename").required(true))
        .arg(Arg::new("create")
            .required(false)
            .short('c')
            .long("no-create")
            .help("prevents files from being created"))
        .arg(Arg::new("date")
            .required(false)
            .short('d')
            .long("date")
            .takes_value(true)
            .help("takes a value in the format \"yyyy-mm-dd hh:mm:ss\" or \"yyyy/mm/dd hh:mm:ss\""))
        .arg(Arg::new("modify")
            .required(false)
            .short('m')
            .help("only changes the modified time metadata"))
        .arg(Arg::new("access")
            .required(false)
            .short('a')
            .help("only changes the access time metadata"))
        .get_matches();

    run(matches).expect("TODO: panic message");
    Ok(())
}
