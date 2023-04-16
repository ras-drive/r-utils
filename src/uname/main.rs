use clap::{Arg, Command};
use r_utils::uname::run;

fn main() {
    let matches = Command::new("uname")
        .version("1.0.0")
        .author("Sarah Petkovic")
        .about("Print certain system information.  With no OPTION, same as -s.")
        .arg(Arg::new("all").short('a').long("all").help(
            "print all information, in the following order, except omit -p and -i if unknown:",
        ))
        .arg(
            Arg::new("kernel-name")
                .short('s')
                .long("kernel-name")
                .help("print the kernel name"),
        )
        .arg(
            Arg::new("node-name")
                .short('n')
                .long("node-name")
                .help("print the network node hostname"),
        )
        .arg(
            Arg::new("kernel-release")
                .short('r')
                .long("kernel-release")
                .help("print the kernel release"),
        )
        .arg(
            Arg::new("machine")
                .short('m')
                .long("machine")
                .help("print the machine hardware name"),
        )
        .arg(
            Arg::new("processor")
                .short('p')
                .long("processor")
                .help("print the hardware platform (non-portable)"),
        )
        .arg(
            Arg::new("hardware-platform")
                .short('i')
                .long("hardware-platform")
                .help("print the hardware platform (non-portable)"),
        )
        .arg(
            Arg::new("operating-system")
                .short('o')
                .long("operating-system")
                .help("print the operating system"),
        )
        .get_matches();

    run(matches);
}

