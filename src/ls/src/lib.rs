pub mod lib {
    use walkdir::WalkDir;

    // Result<(), dyn std::error::Error>
    pub fn search(dir_name: Option<String>, depth: Option<usize>) -> () {
        match &dir_name {
            None => {
                for entry in WalkDir::new("./").max_depth(1) {
                    println!("{}", entry.unwrap().path().display());
                }
            }
            Some(name) => {
                match depth {
                    Some(d) => {
                        for entry in WalkDir::new(name.to_string()).max_depth(d).into_iter().filter_map(|e| e.ok()) {
                            println!("{}", entry.path().display());
                        }
                    }
                    None => {
                        for entry in WalkDir::new(name.to_string()).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                            println!("{}", entry.path().display());
                        }
                    }
                }
                ()
            }
        }
    }
}
