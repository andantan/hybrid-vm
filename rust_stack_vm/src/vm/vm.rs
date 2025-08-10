use std::ffi::c_int;
use crate::vm::executable::Executable;
use crate::vm::op::OpCode;
use crate::stack::composite_stack::CompositeStackI32;
use crate::stack::stack_component::{Stack, StackError};

pub struct VM {
    stack: CompositeStackI32,
    instructions: Vec<OpCode>,
    ip: usize
}

impl VM {
    pub fn new(instructions: Vec<OpCode>) -> Self {
        VM {
            stack: CompositeStackI32::new(1 << 10),
            instructions,
            ip: 0,
        }
    }
}

impl Executable for VM {
    fn run(&mut self) -> Result<c_int, StackError> {
        loop {
            if self.ip >= self.instructions.len() {
                return Ok(self.stack.pop()? as c_int)
            }

            let opcode = &self.instructions[self.ip];
            self.ip += 1;

            match opcode {
                OpCode::PUSH(v) => {
                    match self.stack.push(*v) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    };
                },
                OpCode::ADD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;

                    self.stack.push(a + b)?;
                },
                OpCode::HALT => {
                    return Ok(self.stack.pop()? as c_int);
                }
            }
        }
    }
}