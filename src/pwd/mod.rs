use clap::ArgMatches;
use std::{env, io, path::PathBuf};

pub fn run(matches: ArgMatches) {
    if !matches.is_present("logical") && !matches.is_present("physical") {
        println!("{}", physical_path().unwrap().to_str().unwrap())
    } else if matches.is_present("logical") && !matches.is_present("physical") {
        print!("{}", logical_path().unwrap().to_str().unwrap())
    } else if !matches.is_present("logical") && matches.is_present("physical") {
        println!("{}", physical_path().unwrap().to_str().unwrap())
    }
}
fn physical_path() -> io::Result<PathBuf> {
    // std::env::current_dir() is a thin wrapper around libc::getcwd().
    #[cfg(unix)]
    {
        env::current_dir()
    }
    // On Windows we have to resolve it.
    // On other systems we also resolve it, just in case.
    #[cfg(not(unix))]
    {
        env::current_dir().and_then(|path| path.canonicalize())
    }
}

pub fn logical_path() -> io::Result<PathBuf> {
    // getcwd() on Windows seems to include symlinks, so this is easy.
    #[cfg(windows)]
    {
        env::current_dir()
    }

    // If we're not on Windows we do things Unix-style.

    // The logical working directory is maintained by the shell, in the $PWD
    // environment variable. So we check carefully if that variable looks
    // reasonable, and if not then we fall back to the physical path.
    #[cfg(not(windows))]
    {
        use std::path::Path;
        fn looks_reasonable(path: &Path) -> bool {
            // First, check if it's an absolute path.
            if !path.has_root() {
                return false;
            }

            // Then, make sure there are no . or .. components.
            // Path::components() isn't useful here, it normalizes those out.
            if path
                .to_string_lossy()
                .split(std::path::is_separator)
                .any(|piece| piece == "." || piece == "..")
            {
                return false;
            }

            // Finally, check if it matches the directory we're in.
            #[cfg(unix)]
            {
                use std::fs::metadata;
                use std::os::unix::fs::MetadataExt;
                let path_info = match metadata(path) {
                    Ok(info) => info,
                    Err(_) => return false,
                };
                let real_info = match metadata(".") {
                    Ok(info) => info,
                    Err(_) => return false,
                };
                if path_info.dev() != real_info.dev() || path_info.ino() != real_info.ino() {
                    return false;
                }
            }

            #[cfg(not(unix))]
            {
                use std::fs::canonicalize;
                let canon_path = match canonicalize(path) {
                    Ok(path) => path,
                    Err(_) => return false,
                };
                let real_path = match canonicalize(".") {
                    Ok(path) => path,
                    Err(_) => return false,
                };
                if canon_path != real_path {
                    return false;
                }
            }

            true
        }

        match env::var_os("PWD").map(PathBuf::from) {
            Some(value) if looks_reasonable(&value) => Ok(value),
            _ => env::current_dir(),
        }
    }
}
