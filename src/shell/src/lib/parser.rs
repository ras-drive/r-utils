use std::error::Error;
use crate::lib::Token;

pub struct Parser<'a> {
    data: Vec<Token<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(data: Vec<Token<'a>>) -> Self {
        Parser {
            data
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
            let count = 0;
            let iter = self.data.iter().clone();
            for i in iter.clone() {
                // export parser handle
                if i.to_string() == "export" {
                    let mut token = self.data[count + 1].to_string();
                    token = token.strip_prefix("Text(\"").unwrap().to_string();
                    token = token.strip_suffix("\")").unwrap().to_string();
                    token = token.replace("\\\"", "\"");

                    let key = token.rsplit_once("=").unwrap().0;
                    let text = token.rsplit_once("=").unwrap().1
                        .strip_prefix("\"").unwrap()
                        .strip_suffix("\"").unwrap();

                    std::env::set_var(key, text);
                }

            }
        Ok(())
    }
}