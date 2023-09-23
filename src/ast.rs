#[derive(Debug)]
pub enum UnaryExpressionType {
    Not,
    Negate,
}

#[derive(Debug)]
pub enum BinaryExpressionType {
    Plus,
    Minus,
    Times,
    Divide,
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
}

#[derive(Debug)]
pub enum Expression {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
    Unary(UnaryExpressionType, Box<Expression>),
    Binary(BinaryExpressionType, Box<Expression>, Box<Expression>),
    Paren(Box<Expression>),
}
