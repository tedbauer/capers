use crate::ast::Expression;
use crate::scanner::Token;

pub fn parse(tokens: &Vec<Token>) -> Expression {
    Expression::Number(5.0)
}
