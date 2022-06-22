use std::error::Error;

pub struct Envs<'a> {
    vec: Vec<&'a str>
}

impl<'a> Envs<'a> {
    pub fn new() -> Envs<'a> {
        Envs { vec: Vec::new() }
    }

    pub fn collect(&mut self, vec: Vec<&'a str>) {
        for i in vec {
            self.vec.push(i);
        }
    }

    pub fn push(&mut self, str: &'a str) -> Result<(), Box<dyn Error>> {
        for i in &self.vec {
            if &str == i {
                Err(format!("env item: {} already found", i))?
            }
        }
        self.vec.push(str);
        Ok(())
    }

    pub fn remove(&mut self, str: &'a str) -> Result<&'a str, Box<dyn Error>> {
        // flag to check if it is in vector
        for i in 0..self.vec.capacity() {
            if self.vec[i] == str {
                return Ok(self.vec.remove(i));
            }
        }
        Err(format!("element: {} not found in vector", str))?
    }
}