use std::collections::HashMap;
use crate::ext_hash::ext_hash::{ExtHash};

pub mod ext_hash {
    use std::collections::HashMap;

    /*
    #[derive(Hash, Eq, PartialEq, Debug)]
    pub struct File {
        name: String,
        extension: String
    }

    impl File {
        pub fn new(name: &str, extension: &str) -> File {
            File {name: name.to_string(), extension: name.to_string().split(".").nth(2).unwrap().to_string()}
        }

        pub fn to_string(self) -> String {
            format!("{}.{}", self.name, self.extension)
        }
    }

 */


    pub struct ExtHash {
        data: HashMap<String, String>,
    }

    impl ExtHash {
        pub fn new() -> ExtHash { ExtHash { data: HashMap::new() } }

        pub fn push(mut self, key: String, value: String) -> () {
            self.data.insert(key, value);
        }

        pub fn copy(self: Self) -> ExtHash {
            ExtHash { data: self.data }
        }

        pub fn len(self: Self) -> usize {
            self.data.len()
        }

        pub fn split(self: Self) -> Vec<(String, String)> {
            let mut list: Vec<(String, String)> = Vec::new();
            for i in self.data {
                list.push(i);
            }
            list
        }
    }
}
