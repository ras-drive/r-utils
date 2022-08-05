mod arg_parser;
mod file_list;
mod syntax;

use clap::{Arg, ArgMatches, Command};
use walkdir::WalkDir;

use arg_parser::Args; // arg_parser::arg_parser::Args;
use file_list::FileList; // file_list::file_list::FileList;
use syntax::{get_long, get_metadata}; // {get_long, get_metadata};

pub fn search(dir_name: Option<String>, depth: Option<usize>) -> Vec<String> {
    let mut vec = vec![];

    match &dir_name {
        None => {
            for entry in WalkDir::new("./").max_depth(1) {
                // println!("{}", entry.unwrap().path().display());
                vec.push(entry.expect("").path().display().to_string());
            }
            vec
        }
        Some(name) => {
            match depth {
                Some(d) => {
                    for entry in WalkDir::new(name.to_string())
                        .max_depth(d)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        // println!("{}", entry.path().iter().last().unwrap().to_os_string().into_string().unwrap());
                        vec.push(
                            entry
                                .path()
                                .iter()
                                .last()
                                .unwrap()
                                .to_os_string()
                                .into_string()
                                .unwrap(),
                        );
                    }
                }
                None => {
                    for entry in WalkDir::new(name.to_string())
                        .max_depth(1)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        // println!("{}", entry.path().display());
                        vec.push(entry.path().display().to_string());
                    }
                }
            }
            vec
        }
    }
}

pub fn run(matches: ArgMatches) -> () {
    let mut args = Args::new(matches);
    let mut fl = FileList::new();
    fl.collect(search(Some(args.dir_name), Some(args.depth)));

    if args.long {
        fl.print_long();
    } else {
        fl.print();
    }
}
