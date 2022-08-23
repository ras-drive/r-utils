use clap::{Arg, ArgMatches, Command};
use platform_info::*;

fn main() {
    let matches = Command::new("uname")
        .version("0.1.0")
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

fn run(matches: ArgMatches) {
    let sys = PlatformInfo::new().expect("error while getting platform info");
    if matches.is_present("all") || matches.is_present("kernel-name") || !matches.args_present() {
        print!("{} ", sys.sysname())
    }
    if matches.is_present("all") || matches.is_present("node-name") {
        print!("{} ", sys.nodename())
    }
    if matches.is_present("all") || matches.is_present("kernel-release") {
        print!("{} ", sys.release())
    }
    if matches.is_present("all") || matches.is_present("machine") {
        print!("{} ", sys.machine())
    }
    if matches.is_present("all") || matches.is_present("processor") {
        print!("{} ", sys.machine())
    }
    if matches.is_present("hardware-platform") {
        print!("{} ", sys.machine())
    }
    if matches.is_present("all") || matches.is_present("operating-system") {
        print!("{} ", sys.sysname())
    }
    println!();
}
