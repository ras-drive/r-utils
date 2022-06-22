use std::error::Error;
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use walkdir::{DirEntry, WalkDir};

pub fn search(matches: ArgMatches, recursive: bool) -> Result<Vec<DirEntry>, Box<dyn Error>>{
    let mut max_depth = 1;
    if recursive {
        max_depth = usize::MAX;
    }

    let mut vec = vec![];

    for entry in WalkDir::new("./").min_depth(1).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
        // println!("{}", entry.path().canonicalize().unwrap().display());
        vec.push(entry);
    }

    Ok(vec)
}