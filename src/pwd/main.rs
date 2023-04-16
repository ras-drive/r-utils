use clap::{Arg, Command};
use r_utils::pwd::run;

///
/// pwd
///
/// 0.1.0
/// Description: print name of current/working directory
///
/// Args:
/// -L, --logical: use PWD from environment, even if it contains symlinks
/// -P, --physical: avoid all symlinks
///
/// Default is physical
///
fn main() {
    let matches = Command::new("pwd")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("print name of current/working directory")
        .arg(
            Arg::new("logical")
                .short('L')
                .long("logical")
                .help("use PWD from environment, even if it contains symlinks"),
        )
        .arg(
            Arg::new("physical")
                .short('P')
                .long("physical")
                .help("avoid all symlinks"),
        )
        .get_matches();

    run(matches);
}

