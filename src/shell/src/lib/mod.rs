mod parser;
mod token;

use std::error::Error;
use std::fs::read_to_string;

use crate::lib::parser::Parser;
use crate::lib::token::Token;
use logos::{Lexer, Logos};

pub fn setup(config_name: &str) -> Result<(), Box<dyn Error>> {
    match read_to_string(config_name) {
        Ok(data) => {
            let lex: Lexer<Token> = Token::lexer(data.as_str());
            let mut token_list = vec![];

            for i in lex {
                if i != Token::Error {
                    token_list.push(i);
                }
            }

            let mut parser = Parser::new(token_list);
            parser.run().expect("TODO: Error message");

            Ok(())
        }
        Err(_) => Err(Box::from("Error: config file not found")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, File};
    use std::io::Write;
    #[test]
    fn test_simple_script() {
        let content = String::from("export TEST=\"test\"");
        let mut file = File::create(".shellrc_temp").unwrap();
        file.write(content.as_bytes())
            .expect("error writing test .shellrc config");
        setup(".shellrc_temp").unwrap();

        remove_file(".shellrc_temp").unwrap();
    }
}
