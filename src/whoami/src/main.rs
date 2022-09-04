use clap::Command;
use users::get_current_username;

fn main() {
    let _matches = Command::new("whoami")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about(
            "Print the user name associated with the current effective user ID.  Same as id -un.",
        )
        .get_matches();

    println!("{}", get_current_username().unwrap().to_str().unwrap())
}
