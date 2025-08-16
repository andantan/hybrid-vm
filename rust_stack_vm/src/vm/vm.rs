use crate::vm::op::OpCode;
use crate::stack::composite_stack::{CompositeStack, StackValue};
use crate::stack::stack_component::{Stack, StackError};

pub trait VirtualMachine {
    fn execute(&mut self) -> Result<StackValue, StackError>;
}

pub struct VM {
    stack: CompositeStack,
    instructions: Vec<OpCode>,
    ip: usize
}

impl VM {
    pub fn new(stack_size: usize, instructions: Vec<OpCode>) -> Self {
        assert!(0 < stack_size);

        VM {
            stack: CompositeStack::new(stack_size),
            instructions,
            ip: 0,
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        println!("VM memory has been deallocated.");
    }
}

impl VirtualMachine for VM {
    fn execute(&mut self) -> Result<StackValue, StackError> {
        loop {
            if self.ip >= self.instructions.len() {
                return Ok(self.stack.pop()?)
            }

            let opcode = &self.instructions[self.ip];
            self.ip += 1;

            match opcode {
                OpCode::HALT => {
                    return Ok(self.stack.pop()?);
                },
                OpCode::PUSHINT(i) => {
                    self.stack.push(StackValue::Integer(*i))?;
                },
                OpCode::PUSHBYTE(b) => {
                    self.stack.push(StackValue::Byte(*b))?;
                },
                OpCode::PUSHFLOAT(f) => {
                    self.stack.push(StackValue::Float(*f))?;
                },
                OpCode::POP => {
                    self.stack.pop()?;
                },
                OpCode::ADD => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    match (v1, v2) {
                        // i32 + i32
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => {
                            self.stack.push(StackValue::Integer(i1 + i2))?;
                        },
                        // f32 + f32
                        (StackValue::Float(f1), StackValue::Float(f2)) => {
                            self.stack.push(StackValue::Float(f1 + f2))?;
                        },
                        // i32 + f32 = f32(i32) + f32
                        (StackValue::Integer(i), StackValue::Float(f)) => {
                            self.stack.push(StackValue::Float((i as f32) + f))?;
                        },
                        // f32 + i32 = f32 + f32(i32)
                        (StackValue::Float(f), StackValue::Integer(i)) => {
                            self.stack.push(StackValue::Float(f + (i as f32)))?;
                        }
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                },
                OpCode::SUB => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    match (v1, v2) {
                        // i32 - i32
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => {
                            self.stack.push(StackValue::Integer(i1 - i2))?;
                        },
                        // f32 - f32
                        (StackValue::Float(f1), StackValue::Float(f2)) => {
                            self.stack.push(StackValue::Float(f1 - f2))?;
                        },
                        // i32 - f32 = f32(i32) - f32
                        (StackValue::Integer(i), StackValue::Float(f)) => {
                            self.stack.push(StackValue::Float((i as f32) - f))?;
                        },
                        // f32 - i32 = f32 - f32(i32)
                        (StackValue::Float(f), StackValue::Integer(i)) => {
                            self.stack.push(StackValue::Float(f - (i as f32)))?;
                        }
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                },
                OpCode::MUL => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    match (v1, v2) {
                        // i32 * i32
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => {
                            self.stack.push(StackValue::Integer(i1 * i2))?;
                        },
                        // f32 * f32
                        (StackValue::Float(f1), StackValue::Float(f2)) => {
                            self.stack.push(StackValue::Float(f1 * f2))?;
                        },
                        // i32 * f32 = f32(i32) * f32
                        (StackValue::Integer(i), StackValue::Float(f)) |
                        (StackValue::Float(f), StackValue::Integer(i)) => {
                            self.stack.push(StackValue::Float((i as f32) * f))?;
                        },
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                },
                OpCode::DIV => {
                    let v1 = self.stack.pop()?; // dividend
                    let v2 = self.stack.pop()?; // divisor

                    match (v1, v2) {
                        // i32 / i32
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => {
                            if i2 == 0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Integer(i1 / i2))?;
                        },
                        // f32 / f32
                        (StackValue::Float(f1), StackValue::Float(f2)) => {
                            if f2 == 0.0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Float(f1 / f2))?;
                        },
                        // f32 / i32
                        (StackValue::Integer(i1), StackValue::Float(f2)) => {
                            if f2 == 0.0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Float((i1 as f32) / f2))?;
                        },
                        // i32 / f32
                        (StackValue::Float(f1), StackValue::Integer(i2)) => {
                            if i2 == 0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Float(f1 / (i2 as f32)))?;
                        },
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                }
                OpCode::EQ => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    let result = match (v1, v2) {
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => i1 == i2,
                        (StackValue::Byte(b1), StackValue::Byte(b2)) => b1 == b2,
                        (StackValue::Float(f1), StackValue::Float(f2)) => f1 == f2,
                        (StackValue::Integer(i), StackValue::Float(f)) |
                        (StackValue::Float(f), StackValue::Integer(i)) => (i as f32) == f,
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Integer(if result { 1 } else { 0 }))?;
                },
                OpCode::LT => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    let result = match (v1, v2) {
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => i1 < i2,
                        (StackValue::Byte(b1), StackValue::Byte(b2)) => b1 < b2,
                        (StackValue::Float(f1), StackValue::Float(f2)) => f1 < f2,
                        (StackValue::Integer(i), StackValue::Float(f)) |
                        (StackValue::Float(f), StackValue::Integer(i)) => (i as f32) < f,
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Integer(if result { 1 } else { 0 }))?;
                },
                OpCode::LTE => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    let result = match (v1, v2) {
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => i1 <= i2,
                        (StackValue::Byte(b1), StackValue::Byte(b2)) => b1 <= b2,
                        (StackValue::Float(f1), StackValue::Float(f2)) => f1 <= f2,
                        (StackValue::Integer(i), StackValue::Float(f)) |
                        (StackValue::Float(f), StackValue::Integer(i)) => (i as f32) <= f,
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Integer(if result { 1 } else { 0 }))?;
                },
                OpCode::GT => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    let result = match (v1, v2) {
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => i1 > i2,
                        (StackValue::Byte(b1), StackValue::Byte(b2)) => b1 > b2,
                        (StackValue::Float(f1), StackValue::Float(f2)) => f1 > f2,
                        (StackValue::Integer(i), StackValue::Float(f)) |
                        (StackValue::Float(f), StackValue::Integer(i)) => (i as f32) > f,
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Integer(if result { 1 } else { 0 }))?;
                },
                OpCode::GTE => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    let result = match (v1, v2) {
                        (StackValue::Integer(i1), StackValue::Integer(i2)) => i1 >= i2,
                        (StackValue::Byte(b1), StackValue::Byte(b2)) => b1 >= b2,
                        (StackValue::Float(f1), StackValue::Float(f2)) => f1 >= f2,
                        (StackValue::Integer(i), StackValue::Float(f)) |
                        (StackValue::Float(f), StackValue::Integer(i)) => (i as f32) >= f,
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Integer(if result { 1 } else { 0 }))?;
                }
            }

            println!("{:?}", self.stack);
        }
    }
}