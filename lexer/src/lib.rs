
use std::error::Error;
use std::fs;
use std::fmt::Display;
use std::fmt;

const AND_OPERATOR: char = '&';
const OR_OPERATOR: char = '|';
const NOT_OPERATOR: char = '!';
const L_BRACKET: char = '(';
const R_BRACKET: char = ')';
const WHITESPACE: char = ' ';
const MIN_VAR_CHAR: char = 'A';
const MAX_VAR_CHAR: char = 'z';

#[derive(Debug)]
pub enum Token {
    AndOperator,
    OrOperator,
    NotOperator,
    LBracket,
    RBracket,
    Var(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::AndOperator => write!(f, "{AND_OPERATOR}"),
            Token::OrOperator => write!(f, "{OR_OPERATOR}"),
            Token::NotOperator => write!(f, "{NOT_OPERATOR}"),
            Token::LBracket => write!(f, "{L_BRACKET}"),
            Token::RBracket => write!(f, "{R_BRACKET}"),
            Token::Var(var) => write!(f, "{var}"),
        }
    }
}
pub struct TokenStream(Vec<Token>);

impl Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        let mut iter = self.iter();
        while let Some(token) = iter.next() {
            string.push_str(format!("'{token}'").as_str());
            string.push_str(" ");
        }
        write!(f, "{}", string)
    }
}

impl TokenStream {
    pub fn iter(&self) -> impl Iterator<Item=&Token>{
          self.0.iter()
    }
    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }
    pub fn pop(&mut self) -> Option<Token> {
        self.0.pop()
    }
    pub fn new() -> TokenStream {
        TokenStream(Vec::new())
    }
}

#[derive(Debug)]
pub struct LexerError {
    message: String,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LexerError {}

pub fn run(filename: String) -> Result<TokenStream, Box<dyn Error>> {

    let formula = fs::read_to_string(filename)?;
    let mut stream = TokenStream::new();
    
    for char in formula.chars() {
        let token = match char {
            AND_OPERATOR => Token::AndOperator,
            OR_OPERATOR => Token::OrOperator,
            NOT_OPERATOR => Token::NotOperator,
            L_BRACKET => Token::LBracket,
            R_BRACKET => Token::RBracket,
            MIN_VAR_CHAR ..= MAX_VAR_CHAR => Token::Var(String::from(char)),
            WHITESPACE => continue,
            _ => return Err(Box::new(LexerError { message: format!("Unknown Token: {char}") })),
        };
        stream.push(token);
    }
    Ok(stream)
}