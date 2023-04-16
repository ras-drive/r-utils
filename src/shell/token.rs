use logos::Logos;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    // Tokens can be literal strings, of any length.
    #[token("export")]
    Export,

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

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
