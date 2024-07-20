
use crate::lexer::{TokenStream, Token::{self, *}};
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Clone)]
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

pub fn run_parser(stream: TokenStream) -> Result<Expression, ParserError>  {
    stream.polish_notation()?.expression()
}

impl Token {
    fn precedence(&self) -> u32 {
        match self {
            RBracket => 0,
            OrOperator => 1, 
            AndOperator => 2, 
            NotOperator => 3, 
            _ => panic!("This token {self} should not be in the operator stack!"),
        }
    }
}

impl TokenStream {
    fn polish_notation(mut self) -> Result<TokenStream, ParserError> {
        let mut result = TokenStream::new();
        let mut operators: Vec<Token> = Vec::new();
    
        'main: while let Some(token) = self.pop() {
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
                        if top.precedence() < token.precedence() {
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
    
    fn expression(mut self) -> Result<Expression, ParserError> {
        let expression = self.take_expression()?;
    
        match self.pop() {
            None => Ok(expression),
            Some(_) => Err(ParserError { message: String::from("Malformed Formula") }),
        }
    }
    
    fn take_expression(&mut self) -> Result<Expression, ParserError> {
        match self.pop() {
            None => Err(ParserError { message: String::from("Malformed Formula") }),
            Some(token) => match token {
                Var(var) => Ok(Expression::Var(var)),
                NotOperator => {
                    let expr = self.take_expression()?;
                    Ok(Expression::Not(Box::new(expr)))
                }
                AndOperator => {
                    let expr = self.take_expression()?;
                    let expr2 = self.take_expression()?;
                    Ok(Expression::And(Box::new(expr), Box::new(expr2)))
                }
                OrOperator => {
                    let expr = self.take_expression()?;
                    let expr2 = self.take_expression()?;
                    Ok(Expression::Or(Box::new(expr), Box::new(expr2)))
                }
                _ => panic!("There should not be any brackets at this point!"),
            },
        }
    }
}