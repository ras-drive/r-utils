pub mod lib {
    use walkdir::WalkDir;

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
                        for entry in WalkDir::new(name.to_string()).max_depth(d).into_iter().filter_map(|e| e.ok()) {
                            // println!("{}", entry.path().iter().last().unwrap().to_os_string().into_string().unwrap());
                            vec.push(entry.path().iter().last().unwrap().to_os_string().into_string().unwrap());
                        }
                    }
                    None => {
                        for entry in WalkDir::new(name.to_string()).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                            // println!("{}", entry.path().display());
                            vec.push(entry.path().display().to_string());
                        }
                    }
                }
                vec
            }
        }
    }
}
