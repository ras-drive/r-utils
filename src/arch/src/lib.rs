use platform_info::*;

pub fn get_arch() -> Result<(), Box<dyn std::error::Error>> {
    let sys = PlatformInfo::new().expect("error while getting platform info");
    println!("{}", sys.machine());
    Ok(())
}