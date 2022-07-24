use std::fmt::{Debug, Formatter, Result, Display};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    // Tokens can be literal strings, of any length.
    #[token("export")]
    Keyword(&'a str),

    // Or regular expressions.
    #[regex("[a-zA-Z.=\"]+")]
    Text(&'a str),

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub enum Keyword {
    Export,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}