use super::syntax::{get_long, get_metadata};

pub struct FileList {
    data: Vec<String>,
}

impl FileList {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, string: String) {
        self.data.push(string);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.data.pop()
    }

    pub fn collect(&mut self, mut vec: Vec<String>) -> () {
        for i in vec {
            self.push(i);
        }
    }

    pub fn find(&mut self, string: &str) -> Option<String> {
        for i in &self.data {
            if i.contains(string) {
                return Some(i.clone());
            }
        }
        None
    }

    /*
    pub fn sort_by_alpha_asc(&mut self) -> () {
        self.data.sort()
    }
     */

    pub fn get_data(self) -> Vec<String> {
        self.data
    }

    pub fn print(&self) -> () {
        for i in &self.data {
            println!("{}", i);
        }
    }

    pub fn print_long(&self) -> () {
        for i in &self.data {
            let metadata = get_metadata(i.as_str());
            let contents = get_long(metadata, i.as_str());
            println!("{}", contents);
        }
    }
}
