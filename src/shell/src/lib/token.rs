use std::fmt::{Debug, Formatter, Result, Display};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    // Tokens can be literal strings, of any length.
    // Or regular expressions.
    #[token("export")]
    Keyword(&'a str),

    #[regex("[a-zA-Z.=\"]+")]
    Text(&'a str),

    // Logos requires one token variant to handle errors,
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}