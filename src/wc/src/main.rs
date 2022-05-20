extern crate core;

use std::fmt::Error;
use std::fs::{read_link, File};
use std::io::{self, stdin, BufRead, Bytes, Read, Stdin};
use std::path::{Path, PathBuf};
use std::{file, fs};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("wc")
        .version("0.01")
        .author("Sarah Petkovic")
        .about("A simple wc clone in Rust");
    //.arg("c");

    let x = true;
    if x {
        let x = read_file("test.txt");
        println!("{}", x);
    } else {
        loop {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                println!("{}", line.unwrap());
            }
        }
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
