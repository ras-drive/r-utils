pub mod file {
    use crate::file;
    use chrono::{Date, TimeZone};
    use permissions;
    use std::fs;
    use std::fs::Metadata;
    use std::io::{Error, ErrorKind};
    use std::ops::Deref;
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

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

            File {
                file: None,
                name: filename,
                ext: None,
                perms: None,
                owner: None,
                date: None,
                size: None,
            }
        }
        pub fn get_name(self: Self) -> String {
            self.name
        }
        pub fn set_name(self: &mut Self, name: &str) -> () {
            self.name = String::from(name);
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
                Ok(data) => Ok(data),
                Err(e) => {
                    eprintln!("ERROR: METADATA IRRETRIEVABLE FOR FILE");
                    Err(Error::new(
                        ErrorKind::InvalidInput,
                        "ERROR: METADATA IRRETRIEVABLE FOR FILE",
                    ))
                }
            }
        }
        // TODO(): Result<String, std::io::Error>
        pub fn get_perms(self: Self) -> Result<String, ()> {
            if self.perms == None {
                let meta = self.get_metadata().unwrap();
                let perms = meta.permissions().mode();
                let mut perm_string = String::new();
                for i in perms.to_string().chars() {
                    match i.to_string().parse().unwrap() {
                        7 => perm_string.push_str("rwx"),
                        6 => perm_string.push_str("rw-"),
                        5 => perm_string.push_str("r-x"),
                        4 => perm_string.push_str("r--"),
                        3 => perm_string.push_str("-wx"),
                        2 => perm_string.push_str("-w-"),
                        1 => perm_string.push_str("--x"),
                        e => {
                            println!("{:?}", char::from_u32(e as u32).unwrap())
                            // !("error while reading file permissions")
                        }
                    }
                }
                Ok(perm_string)
            } else {
                Ok(self.perms.unwrap())
            }
        }
    }
}
