
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
    pub fn last(&mut self) -> Option<&Token> {
        self.0.last()
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
    Ok(get_tokens(formula)?)
}

enum LexerState {
    Ready,
    InVar(String),
}

fn get_tokens(formula: String) -> Result<TokenStream, LexerError> {
    let mut stream = TokenStream::new();
    let mut state = LexerState::Ready;
    
    for char in formula.chars() {
        match state {
            LexerState::Ready => match char {
                AND_OPERATOR => stream.push(Token::AndOperator),
                OR_OPERATOR => stream.push(Token::OrOperator),
                NOT_OPERATOR => stream.push(Token::NotOperator),
                L_BRACKET => stream.push(Token::LBracket),
                R_BRACKET => stream.push(Token::RBracket),
                WHITESPACE => continue,
                MIN_VAR_CHAR ..= MAX_VAR_CHAR => state = LexerState::InVar(String::from(char)),
                _ => return Err(LexerError { message: format!("Unknown Token: {char}") }),
            },
            LexerState::InVar(var) => match char {
                AND_OPERATOR | OR_OPERATOR | R_BRACKET | WHITESPACE => {
                    stream.push(Token::Var(var));
                    match char {
                        AND_OPERATOR => stream.push(Token::AndOperator),
                        OR_OPERATOR => stream.push(Token::OrOperator),
                        R_BRACKET => stream.push(Token::RBracket),
                        WHITESPACE => (),
                        _ => panic!("Should never be called!"),
                    }
                    state = LexerState::Ready;
                },
                MIN_VAR_CHAR ..= MAX_VAR_CHAR => state = LexerState::InVar(format!("{var}{char}")),
                NOT_OPERATOR | L_BRACKET => return Err(LexerError { message: format!("Malformed Formula") }),
                _ => return Err(LexerError { message: format!("Unknown Token: {char}") }),
            },
        }
    }

    if let LexerState::InVar(var) = state {
        stream.push(Token::Var(var));
    }

    Ok(stream)
}