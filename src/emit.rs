use crate::scanner::{Token, TokenType};
use crate::vm::{Chunk, OpCode};
use anyhow::{anyhow, Error};
use std::collections::VecDeque;

// push 2
//   push 3
//   push 5
//   push +
// push *
// 2 * 3 + 5

// push 2

// 2 * 3 + 5

fn is_operator(token: &Token) -> bool {
    match token.token_type {
        TokenType::Plus => true,
        TokenType::Minus => true,
        TokenType::Star=> true,
        _ => false,
    }
}

fn try_push_operator(token: &Token, chunk_acc: &mut Chunk) -> Result<(), Error> {
    match token.token_type {
        TokenType::Plus => chunk_acc.code.push(OpCode::Add as u8),
        TokenType::Minus => chunk_acc.code.push(OpCode::Subtract as u8),
        TokenType::Star => chunk_acc.code.push(OpCode::Multiply as u8),
        _ => return Err(anyhow!("expected operator at line {}", token.line)),
    }

    chunk_acc.code.push(OpCode::Return as u8);
    Ok(())
}

fn try_push_value(token: Token, chunk_acc: &mut Chunk) -> Result<(), Error> {
    match token.token_type {
        TokenType::Number => {
            chunk_acc.add_constant(token.lexeme.parse::<f32>().unwrap());
            Ok(())
        }
        _ => Err(anyhow!("expected value at line {}", token.line)),
    }
}

fn op_power(token: &Token) -> Result<u8, Error> {
    match token.token_type {
        TokenType::Plus => Ok(5),
        TokenType::Minus => Ok(5),
        TokenType::Star => Ok(9),
        _ => Err(anyhow!("tried using op_power on non-op token: {:?}", token)),
    }
}

fn expr_emit(
    tokens: &mut VecDeque<Token>,
    chunk_acc: &mut Chunk,
    max_power: u8,
) -> Result<(), Error> {
    // 3 * 2 + 1

    // Assumption: tokens[0] is a value. tokens[1] should be either nothing or an operator.
    // 1. Check next token.
    //    - If it's an operator, and it's less than max_power, stop.
    //    - If it's not an operator, error.
    //    - If it's nothing, stop.
    // 2. Pop the current token. Try to add the value.

    loop {
        let curr_wrapped = tokens.get(0).clone();
        if let None::<&Token> = curr_wrapped {
            return Ok(())
        }
        let curr = curr_wrapped.unwrap();
        if is_operator(&curr) {
            if let Some(t) = tokens.pop_front() {
                expr_emit(tokens, chunk_acc, op_power(&t).unwrap());
                try_push_operator(&t, chunk_acc);
            }
            continue;
        }
        let op = tokens.get(1).clone();
        println!(
            "calling emit with power {}, with curr {:?}, with op {:?}",
            max_power, curr, op
        );
        let mut is_end = false;
        let mut push_more = true;
        match op.clone() {
            Some(op) => {
                if op_power(&op).unwrap() <= max_power {
                    println!("saw {}, returning", op_power(&op).unwrap());
                    push_more = false;
                }
            }
            None => is_end = true,
        }

        match tokens.pop_front() {
            Some(token) => {
                println!("processing {:?}", token);
                try_push_value(token.clone(), chunk_acc)?;
                if is_end || !push_more {
                    return Ok(());
                }
            }
            None => return Ok(()),
        }

        match tokens.pop_front() {
            Some(token) => {
                println!("we just popped this token: {:?}", token);
                expr_emit(tokens, chunk_acc, op_power(&token).unwrap())?;
                try_push_operator(&token, chunk_acc);
            }
            None => return Ok(()),
        }
    }
}

// 3 * 2 + 1

pub fn emit(tokens: Vec<Token>) -> Chunk {
    let mut chunk = Chunk::new();
    let mut tokens = VecDeque::from(tokens);
    println!("the tokens are: {:?}", tokens);
    expr_emit(&mut tokens, &mut chunk, 0).unwrap();
    chunk
}
