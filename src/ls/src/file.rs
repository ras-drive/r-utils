pub mod file {
    use std::fs;
    use std::fs::Metadata;
    use chrono::{Date, TimeZone};
    use permissions;
    use crate::file;
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    use std::io::{Error, ErrorKind};
    use std::ops::Deref;

    pub struct File {
        file: Option<fs::File>,
        name: String,
        ext: Option<String>,
        perms: Option<String>,
        owner: Option<u32>,
        date: Option<String>,
        size: Option<usize>,
    }

    impl File {
        pub fn new(name: Option<String>) -> File {
            let filename;
            match name {
                Some(data) => {
                    filename = data;
                }
                None => {
                    filename = String::from("ERROR: FILE UNUSABLE IN HASH");
                }
            }

            File { file: None, name: filename, ext: None, perms: None, owner: None, date: None, size: None }
        }
        pub fn get_name(self: Self) -> String {
            self.name
        }
        fn get_owner(self: Self) -> std::io::Result<u32> {
            if self.owner == Some(0) {
                let meta = self.get_metadata().unwrap();
                Ok(meta.uid())
            } else {
                Ok(self.owner.unwrap())
            }
        }

        fn get_metadata(self: Self) -> std::io::Result<Metadata> {
            let mut meta = fs::metadata(self.name);
            match meta {
                Ok(data) => { Ok(data) }
                Err(e) => {
                    eprintln!("ERROR: METADATA IRRETRIEVABLE FOR FILE");
                    Err(Error::new(ErrorKind::InvalidInput, "ERROR: METADATA IRRETRIEVABLE FOR FILE"))
                }
            }
        }
        // TODO(): Result<String, std::io::Error>
        fn get_perms(self: Self) -> Result<String, ()> {
            if self.perms == None {
                let meta = self.get_metadata().unwrap();
                let perms = meta.permissions().mode();
                let mut perm_string = String::new();
                for i in perms.to_string().chars() {
                    match i as i32 {
                        7 => { perm_string.push_str("rwx") }
                        6 => { perm_string.push_str("rw-") }
                        5 => { perm_string.push_str("r-x") }
                        4 => { perm_string.push_str("r--") }
                        3 => { perm_string.push_str("-wx") }
                        2 => { perm_string.push_str("-w-") }
                        1 => { perm_string.push_str("--x") }
                        _ => { eprintln!("error while reading file permissions") }
                    }
                }
                Ok(perm_string)
            } else {
                Ok(self.perms.unwrap())
            }
        }
    }
}