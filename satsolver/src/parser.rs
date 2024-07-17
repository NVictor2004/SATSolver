
use crate::lexer::{TokenStream, Token::{self, *}};
use std::error::Error;
use std::fmt::{self, Display};

pub enum Expression {
    Var(String),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Var(var) => write!(f, "{var}"),
            Expression::Not(expr) => {
                write!(f, "!")?;
                Display::fmt(expr, f)
            }
            Expression::And(expr, expr2) => {
                write!(f, "(")?;
                Display::fmt(expr, f)?;
                write!(f, " & ")?;
                Display::fmt(expr2, f)?;
                write!(f, ")")
            }
            Expression::Or(expr, expr2) => {
                write!(f, "(")?;
                Display::fmt(expr, f)?;
                write!(f, " | ")?;
                Display::fmt(expr2, f)?;
                write!(f, ")")
            }
        }
    }
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

    Ok(expression(polish)?)
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

fn expression(mut stream: TokenStream) -> Result<Expression, ParserError> {
    let expression = take_expression(&mut stream)?;

    match stream.pop() {
        None => Ok(expression),
        Some(_) => Err(ParserError { message: String::from("Malformed Formula") }),
    }
}

fn take_expression(stream: &mut TokenStream) -> Result<Expression, ParserError> {
    match stream.pop() {
        None => Err(ParserError { message: String::from("Malformed Formula") }),
        Some(token) => match token {
            Var(var) => Ok(Expression::Var(var)),
            NotOperator => {
                let expr = take_expression(stream)?;
                Ok(Expression::Not(Box::new(expr)))
            }
            AndOperator => {
                let expr = take_expression(stream)?;
                let expr2 = take_expression(stream)?;
                Ok(Expression::And(Box::new(expr), Box::new(expr2)))
            }
            OrOperator => {
                let expr = take_expression(stream)?;
                let expr2 = take_expression(stream)?;
                Ok(Expression::Or(Box::new(expr), Box::new(expr2)))
            }
            _ => panic!("There should not be any brackets at this point!"),
        },
    }
}