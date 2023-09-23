use std::mem::transmute;

#[derive(Debug)]
#[repr(u8)]
enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    // Subtract,
    // Multiply,
    // Divide,
}

#[derive(Clone)]
pub struct Chunk {
    code: Vec<u8>,
    constant_pool: Vec<f32>,
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
                println!("{:?}", vm.pop());
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
        };
        vm.ip += 1;
    }
    InterpretResult::Ok
}

pub fn execute() {
    let mut code = Vec::new();

    code.push(OpCode::Constant as u8);
    let mut constant_pool: Vec<f32> = Vec::new();
    constant_pool.push(5.4);
    code.push(0);
    code.push(OpCode::Negate as u8);
    code.push(OpCode::Constant as u8);
    constant_pool.push(1.0);
    code.push(1);
    code.push(OpCode::Add as u8);
    code.push(OpCode::Return as u8);

    // disassemble_chunk(&Chunk { code, constant_pool })
    run(&Chunk {
        code,
        constant_pool,
    });
}
