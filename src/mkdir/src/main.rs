use clap::{Arg, Command};
use mkdir::{check_dirs, make_dirs};

// TODO: add the rest of the mkdir arguments
// -m --mode: set file mode (as in chmod), not a=rwx - umask,
// p --parents: no error if existing, make parent directories as needed, with their file modes unaffected by any -m option,
// -Z: set SELinux security context of each created directory to the default type
// --context: like -Z, or if CTX is specified then set the SELinux or SMACK security context to CTX

fn main() {
    let matches = Command::new("mkdir")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("Create the directory(ies), if they do not already exist.")
        .arg(
            Arg::new("directories")
                .required(true)
                .takes_value(true)
                .help("path of directory(ies) to create")
                .multiple_values(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .required(false)
                .takes_value(false)
                .help("print a message for each created directory"),
        )
        .get_matches();

    let dir_paths: Vec<&str> = matches.values_of("directories").unwrap().collect();

    check_dirs(&dir_paths).expect("error while checking to see if supplied dirs exist");
    make_dirs(&matches, &dir_paths).expect("error while creating supplied dirs");
}
