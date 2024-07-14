
use lexer::{TokenStream, Token::{self, *}};
use std::error::Error;
use std::fmt::{self, Display};

pub enum Expression {
    Var(&'static str),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

#[derive(Debug)]
pub struct ParserError {
    message: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParserError {}

pub fn run(stream: TokenStream) -> Result<Expression, ParserError>  {
    let polish = polish_notation(stream)?;
    println!("{polish}");
    Err(ParserError { message: String::new() })
}

fn polish_notation(mut stream: TokenStream) -> Result<TokenStream, ParserError> {
    let mut result = TokenStream::new();
    let mut operators: Vec<Token> = Vec::new();

    'main: while let Some(token) = stream.pop() {
        match token {
            Var(_) => result.push(token),
            RBracket => operators.push(token),
            LBracket => {
                while let Some(operator) = operators.pop() {
                    match operator {
                        RBracket => continue 'main,
                        _ => result.push(operator),
                    }
                }
                return Err(ParserError { message: String::from("Missing )") });
            }
            _ => {
                while let Some(top) = operators.last() {
                    if precedence(top) < precedence(&token) {
                        break;
                    }
                    result.push(operators.pop().unwrap());
                }
                operators.push(token);
            }
        }
    }

    while let Some(operator) = operators.pop() {
        match operator {
            RBracket => return Err(ParserError { message: String::from("Missing (") }),
            _ => result.push(operator),
        }
    }

    Ok(result)
}

fn precedence(token: &Token) -> u32 {
    match token {
        RBracket => 0,
        OrOperator => 1, 
        AndOperator => 2, 
        NotOperator => 3, 
        _ => panic!("This token {token} should not be in the operator stack!"),
    }
}