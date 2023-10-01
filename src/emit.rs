use crate::scanner::{Token, TokenType};
use crate::vm::{Chunk, OpCode};

// 2 + 3 + 4

// 5 + 2 * 3 = 11
// 2 * 4 + 1 = 9

// parse the first character.
// if it's not a number, error.
// check the next char.
// if it's eof, return.
// if it's a plus, recurse on the next char.

fn expr_emit(mut tokens: Vec<Token>, chunk_acc: &mut Chunk) {
    if let Some(token) = tokens.pop() {
        match token.token_type {
            TokenType::Number => {
                chunk_acc.add_constant(token.lexeme.parse::<f32>().unwrap());
            }
            _ => panic!("error"),
        }
    } else {
        return;
    }

    if let (Some(token)) = tokens.pop() {
        match token.token_type {
            TokenType::Plus => {
                expr_emit(tokens, chunk_acc);
                chunk_acc.code.push(OpCode::Add as u8);
                chunk_acc.code.push(OpCode::Return as u8);
            }
            _ => panic!("error"),
        }
    }
}

pub fn emit(tokens: Vec<Token>) -> Chunk {
    let mut chunk = Chunk::new();
    expr_emit(tokens, &mut chunk);
    chunk
}
