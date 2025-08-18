use crate::vm::op::OpCode;
use crate::stack::composite_stack::{CompositeStack, StackValue};
use crate::stack::stack::{Stack, StackError};

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
                OpCode::PACK(u) => {
                    let n = *u as usize;
                    let vals = self.stack.pop_n(n, false)?;
                    let byte_vec_result: Result<Vec<u8>, StackError> = vals.into_iter().map(|v| {
                        if let StackValue::Byte(b) = v {
                            Ok(b)
                        } else {
                            Err(StackError::StackInvalidType)
                        }
                    }).collect();

                    let byte_vec = byte_vec_result?;

                    self.stack.push(StackValue::ByteArray(byte_vec))?;
                },
                OpCode::POP => {
                    self.stack.pop()?;
                },
                OpCode::ADD => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    match (lhs, rhs) {
                        // i32 + i32
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => {
                            self.stack.push(StackValue::Integer(lhs_i32 + rhs_i32))?;
                        },
                        // f32 + f32
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => {
                            self.stack.push(StackValue::Float(lhs_f32 + rhs_f32))?;
                        },
                        // i32 + f32 = f32(i32) + f32
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => {
                            self.stack.push(StackValue::Float((lhs_i32 as f32) + rhs_f32))?;
                        },
                        // f32 + i32 = f32 + f32(i32)
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => {
                            self.stack.push(StackValue::Float(lhs_f32 + (rhs_i32 as f32)))?;
                        }
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                },
                OpCode::SUB => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    match (lhs, rhs) {
                        // i32 - i32
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => {
                            self.stack.push(StackValue::Integer(lhs_i32 - rhs_i32))?;
                        },
                        // f32 - f32
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => {
                            self.stack.push(StackValue::Float(lhs_f32 - rhs_f32))?;
                        },
                        // i32 - f32 = f32(i32) - f32
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => {
                            self.stack.push(StackValue::Float((lhs_i32 as f32) - rhs_f32))?;
                        },
                        // f32 - i32 = f32 - f32(i32)
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => {
                            self.stack.push(StackValue::Float(lhs_f32 - (rhs_i32 as f32)))?;
                        }
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                },
                OpCode::MUL => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    match (lhs, rhs) {
                        // i32 * i32
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => {
                            self.stack.push(StackValue::Integer(lhs_i32 * rhs_i32))?;
                        },
                        // f32 * f32
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => {
                            self.stack.push(StackValue::Float(lhs_f32 * rhs_f32))?;
                        },
                        // i32 * f32 = f32(i32) * f32
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => {
                            self.stack.push(StackValue::Float((lhs_i32 as f32) * rhs_f32))?;
                        },
                        // f32 * i32 = f32 * f32(i32)
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => {
                            self.stack.push(StackValue::Float(lhs_f32 * (rhs_i32 as f32)))?;
                        },
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                },
                OpCode::DIV => {
                    let rhs = self.stack.pop()?; // dividend
                    let lhs = self.stack.pop()?; // divisor

                    match (lhs, rhs) {
                        // i32 / i32
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => {
                            if rhs_i32 == 0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Integer(lhs_i32 / rhs_i32))?;
                        },
                        // f32 / f32
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => {
                            if rhs_f32 == 0.0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Float(lhs_f32 / rhs_f32))?;
                        },
                        // i32 / f32 = f32(i32) / i32
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => {
                            if rhs_f32 == 0.0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Float((lhs_i32 as f32) / rhs_f32))?;
                        },
                        // f32 / i32 = f32 / f32(i32)
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => {
                            if rhs_i32 == 0 { return Err(StackError::DivisionByZero); }

                            self.stack.push(StackValue::Float(lhs_f32 / (rhs_i32 as f32)))?;
                        },
                        _ => {
                            return Err(StackError::StackInvalidType);
                        }
                    }
                }
                OpCode::EQ => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    let result = match (lhs, rhs) {
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => lhs_i32 == rhs_i32,
                        (StackValue::Byte(lhs_u8), StackValue::Byte(rhs_u8)) => lhs_u8 == rhs_u8,
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => lhs_f32 == rhs_f32,
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => (lhs_i32 as f32) == rhs_f32,
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => lhs_f32 == (rhs_i32 as f32),
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Bool(result))?;
                },
                OpCode::LT => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    let result = match (lhs, rhs) {
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => lhs_i32 < rhs_i32,
                        (StackValue::Byte(lhs_u8), StackValue::Byte(rhs_u8)) => lhs_u8 < rhs_u8,
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => lhs_f32 < rhs_f32,
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => (lhs_i32 as f32) < rhs_f32,
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => lhs_f32 < (rhs_i32 as f32),
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Bool(result))?;
                },
                OpCode::LTE => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    let result = match (lhs, rhs) {
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => lhs_i32 <= rhs_i32,
                        (StackValue::Byte(lhs_u8), StackValue::Byte(rhs_u8)) => lhs_u8 <= rhs_u8,
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => lhs_f32 <= rhs_f32,
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => (lhs_i32 as f32) <= rhs_f32,
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => lhs_f32 <= (rhs_i32 as f32),
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Bool(result))?;
                },
                OpCode::GT => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    let result = match (lhs, rhs) {
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => lhs_i32 > rhs_i32,
                        (StackValue::Byte(lhs_u8), StackValue::Byte(rhs_u8)) => lhs_u8 > rhs_u8,
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => lhs_f32 > rhs_f32,
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => (lhs_i32 as f32) > rhs_f32,
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => lhs_f32 > (rhs_i32 as f32),
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Bool(result))?;
                },
                OpCode::GTE => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;

                    let result = match (lhs, rhs) {
                        (StackValue::Integer(lhs_i32), StackValue::Integer(rhs_i32)) => lhs_i32 >= rhs_i32,
                        (StackValue::Byte(lhs_u8), StackValue::Byte(rhs_u8)) => lhs_u8 >= rhs_u8,
                        (StackValue::Float(lhs_f32), StackValue::Float(rhs_f32)) => lhs_f32 >= rhs_f32,
                        (StackValue::Integer(lhs_i32), StackValue::Float(rhs_f32)) => (lhs_i32 as f32) >= rhs_f32,
                        (StackValue::Float(lhs_f32), StackValue::Integer(rhs_i32)) => lhs_f32 >= (rhs_i32 as f32),
                        _ => return Err(StackError::StackInvalidType),
                    };

                    self.stack.push(StackValue::Bool(result))?;
                },
                OpCode::CONCAT => {
                    let v1 = self.stack.pop()?;
                    let v2 = self.stack.pop()?;

                    if let (StackValue::ByteArray(bytes1), StackValue::ByteArray(mut bytes2)) = (v1, v2) {
                        bytes2.extend_from_slice(&bytes1);

                        self.stack.push(StackValue::ByteArray(bytes2))?;
                    } else {
                        return Err(StackError::StackInvalidType);
                    }
                }
            }

            println!("{:?}", self.stack);
        }
    }
}