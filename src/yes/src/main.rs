///
/// ## Name
/// yes - output a string repeatedly until killed
///
/// ## Description
/// The yes command outputs the same string, STRING, in a constant stream.
/// If STRING is not specified, the word it repeats is y.
///
/// ## Arguments
/// OPT `STRING` - the string to output
///

fn main() {
    let matches = clap::App::new("yes")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("output a string repeatedly until killed")
        .arg(clap::Arg::new("string").required(false))
        .get_matches();

    let data = match matches.is_present("string") {
        true => matches.value_of("string").unwrap().to_string(),
        false => String::from("y"),
    };

    loop {
        println!("{}", data);
    }
}
