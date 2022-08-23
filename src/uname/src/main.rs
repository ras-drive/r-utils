use clap::{Command, ArgMatches};
use platform_info::*;

fn main() {
    let matches = Command::new("uname")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("Print certain system information.  With no OPTION, same as -s.")
        .get_matches();

    run(matches);
}

fn run(_matches: ArgMatches) {
    let sys = PlatformInfo::new().expect("error while getting platform info");
    println!("{}", sys.sysname())
}