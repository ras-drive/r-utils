use arch::get_arch;
use clap::Command;

fn main() {
    Command::new("arch")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("print machine hardware name")
        .get_matches();

    get_arch().expect("error while getting system arch");
}
