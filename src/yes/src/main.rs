fn main() {
    let matches = clap::App::new("yes")
        .version("0.1")
        .author("Sarah Petkovic")
        .about("output a string repeatedly until killed")
        .arg(clap::Arg::new("name").required(false))
        .get_matches();

    let data;
    match matches.is_present("name") {
        true => {
            data = matches.value_of("name").unwrap().to_string();
        }
        false => {
            data = String::from("y");
        }
    }

    loop {
        println!("{}", data);
    }
}
