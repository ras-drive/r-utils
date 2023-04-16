use clap::ArgMatches;
use platform_info::*;

pub fn run(matches: ArgMatches) {
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