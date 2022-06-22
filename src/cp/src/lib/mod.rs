use std::fs::{File, read_dir};
use std::io;
use std::path::{Path, PathBuf};

pub fn search_dic(str: &str, canonical: bool) -> Vec<PathBuf> {
    let mut vec = vec![];

    match read_dir(Path::new(str)) {
        Ok(mut data) => {
            for i in data {
                if canonical {
                    vec.push(i.unwrap().path().canonicalize().unwrap());
                } else {
                    vec.push(i.unwrap().path());
                }
            }
        }
        Err(_) => {}
    }
    vec
}