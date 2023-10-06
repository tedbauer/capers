use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;

#[repr(C)]
union Value {
    number: f32,
    boolean: bool,
    nil: (),
}

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    // Divide,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constant_pool: Vec<f32>,

    constant_pool_top: u8,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut byte = 0;
        while byte < self.code.len() {
            let op_code: OpCode = unsafe { transmute::<u8, OpCode>(self.code[byte]) };
            print!("{:?}", op_code);
            match op_code {
                OpCode::Constant => {
                    byte += 1;
                    println!(
                        " : {}",
                        self.constant_pool[self.code[byte as usize] as usize]
                    );
                }
                _ => println!(""),
            }
            byte += 1;
        }
        Ok(())
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constant_pool: Vec::new(),
            constant_pool_top: 0,
        }
    }

    pub fn add_constant(&mut self, constant: f32) {
        self.code.push(OpCode::Constant as u8);
        self.code.push(self.constant_pool_top);

        self.constant_pool.push(constant);
        self.constant_pool_top += 1;
    }
}

struct VirtualMachine {
    chunk: Chunk,
    ip: u8,
    stack: [f32; 256],
    stack_top: usize,
}

impl VirtualMachine {
    fn push(&mut self, val: f32) {
        self.stack[self.stack_top] = val;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> f32 {
        let result = self.stack[self.stack_top - 1];
        self.stack_top -= 1;
        result
    }
}

enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

// fn disassemble_chunk(chunk: &Chunk) {
//   let mut byte = 0;
// while byte < chunk.code.len() {
//   let op_code: OpCode = unsafe { transmute::<u8, OpCode>(chunk.code[byte]) };
// println!("Opcode: {:?}", op_code);
// match op_code {
//   OpCode::Return => (),
// OpCode::Constant => {
//   byte += 1;
// println!("{:?}", chunk.constant_pool[chunk.code[byte] as usize]);
// },
// }
// byte += 1;
// }
// }

fn run(chunk: &Chunk) -> InterpretResult {
    let mut vm = VirtualMachine {
        chunk: chunk.clone(),
        ip: 0,
        stack: [0.0; 256],
        stack_top: 0,
    };
    while (vm.ip as usize) < vm.chunk.code.len() {
        let op_code: OpCode = unsafe { transmute::<u8, OpCode>(vm.chunk.code[vm.ip as usize]) };
        match op_code {
            OpCode::Return => {
                let res = vm.pop();
                println!("{:?}", res);
                vm.push(res);
            }
            OpCode::Constant => {
                vm.ip += 1;
                vm.push(vm.chunk.constant_pool[vm.chunk.code[vm.ip as usize] as usize]);
            }
            OpCode::Negate => {
                let val = vm.pop();
                vm.push(-val);
            }
            OpCode::Add => {
                let a = vm.pop();
                let b = vm.pop();
                vm.push(a + b);
            }
            OpCode::Subtract => {
                let a = vm.pop();
                let b = vm.pop();
                vm.push(a - b);
            }
            OpCode::Multiply => {
                let a = vm.pop();
                let b = vm.pop();
                vm.push(a * b);
            }
        };
        vm.ip += 1;
    }
    InterpretResult::Ok
}

pub fn execute(chunk: Chunk) {
    println!("===\n{}", &chunk);
    run(&chunk);
}
