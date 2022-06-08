use std::collections::HashMap;
use crate::ext_hash::ext_hash::{ExtHash};

pub mod ext_hash {
    use std::collections::HashMap;
    use std::ops::Index;

    pub struct ExtHash {
        data: HashMap<String, String>,
    }

    impl ExtHash {
        pub fn new() -> ExtHash { ExtHash { data: HashMap::new() } }

        pub fn push(mut self, key: String, value: String) -> () {
            self.data.insert(key, value);
        }

        pub fn len(self: Self) -> usize {
            self.data.len()
        }

        pub fn keys(self: Self, index: Option<i32>) -> Vec<String> {
            let mut key_vec: Vec<String> = vec![];
            match index {
                Some(data) => {
                    let mut count = 0;
                    for i in self.data {
                        if count == data {
                            key_vec.push(i.0)
                        } else {
                            count += 1;
                        }
                    }
                    if key_vec.is_empty() {
                        eprintln!("error while grabbing the keys");
                    }
                }
                None => {
                    for i in self.data {
                        key_vec.push(i.0);
                    }
                }
            }
            key_vec
        }
        pub fn values(self: Self, index: Option<i32>) -> Vec<String> {
            let mut key_vec: Vec<String> = vec![];
            match index {
                Some(data) => {
                    let mut count = 0;
                    for i in self.data {
                        if count == data {
                            key_vec.push(i.1)
                        } else {
                            count += 1;
                        }
                    }
                    if key_vec.is_empty() {
                        eprintln!("error while grabbing the keys");
                    }
                }
                None => {
                    for i in self.data {
                        key_vec.push(i.1);
                    }
                }
            }
            key_vec
        }
    }
}
