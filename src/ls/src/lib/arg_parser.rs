
use clap::ArgMatches;

pub struct Args {
    pub dir_name: String,
    pub depth: usize,
    pub long: bool,
}

impl Args {
    pub fn new(matches: ArgMatches) -> Args {
        let mut dir_name = String::from("./");
        let mut depth = 1;
        let mut long = false;

        if matches.is_present("depth") {
            depth = matches
                .value_of("depth")
                .expect("")
                .parse()
                .expect("Error while reading depth argument value");
        }

        if matches.is_present("dir_name") {
            dir_name = matches
                .value_of("dir_name")
                .expect("error while reading provided directory name")
                .parse()
                .expect("Error while reading directory name argument value");
        }

        if matches.is_present("long") {
            long = true;
        }

        Args {
            dir_name,
            depth,
            long,
        }
    }
}
