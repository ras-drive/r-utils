use clap::Command;
use r_utils::arch::get_arch;

fn main() {
    Command::new("arch")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("print machine hardware name")
        .get_matches();

    get_arch().expect("error while getting system arch");
}
