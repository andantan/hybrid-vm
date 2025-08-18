mod vm;
mod stack;

use std::slice;
use vm::vm::VM;
use vm::op::{OpCode, Operation};
use stack::stack::StackError;
use crate::stack::composite_stack::StackValue;
use crate::vm::result::VMResult;
use crate::vm::vm::VirtualMachine;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_vm(
    stack_size: usize,
    instruction_ptr: *const Operation,
    instruction_len: usize,
) -> *mut VM {
    let instruction_slice = unsafe {
        slice::from_raw_parts(instruction_ptr, instruction_len)
    };
    
    let instructions: Vec<OpCode> = instruction_slice.iter().map(|opcode| {
        match opcode.kind {
            0x00 => OpCode::HALT,
            0x01 => OpCode::PUSHINT( unsafe { opcode.val.int_val } ),
            0x02 => OpCode::PUSHFLOAT( unsafe { opcode.val.float_val } ),
            0x03 => OpCode::PUSHBYTE( unsafe { opcode.val.byte_val } ),
            0x04 => OpCode::PACK( unsafe { opcode.val.uint_val } ),
            0x05 => OpCode::POP,
            0x06 => OpCode::ADD,
            0x07 => OpCode::SUB,
            0x08 => OpCode::MUL,
            0x09 => OpCode::DIV,
            0x0A => OpCode::EQ,
            0x0B => OpCode::GT,
            0x0C => OpCode::GTE,
            0x0D => OpCode::LT,
            0x0E => OpCode::LTE,
            0x0F => OpCode::CONCAT,
            _ => panic!("Unknown opcode: {}", opcode.kind),
        }
    }).collect();

    let vm = VM::new(stack_size, instructions);

    Box::into_raw(Box::new(vm))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_vm(vm_ptr: *mut VM) -> VMResult {
    let vm = unsafe {
        &mut *vm_ptr
    };

    match vm.execute() {
        Ok(stack_value) => match stack_value {
            StackValue::Integer(i) => VMResult::ok_int(i),
            StackValue::Float(f) => VMResult::ok_float(f),
            StackValue::Byte(b) => VMResult::ok_byte(b),
            StackValue::ByteArray(vec) => VMResult::ok_byte_array(vec),
            StackValue::Bool(b) => VMResult::ok_bool(b),
        },
        Err(e) => {
            match e {
                StackError::StackUnderFlow => VMResult::err(0),
                StackError::StackOverFlow => VMResult::err(1),
                StackError::StackInvalidType => VMResult::err(2),
                StackError::DivisionByZero => VMResult::err(3),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_byte_array(ptr: *mut u8, len: usize, capacity: usize) {
    if !ptr.is_null() {
        let _ = unsafe { Vec::from_raw_parts(ptr, len, capacity) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_vm(vm_ptr: *mut VM) {
    if !vm_ptr.is_null() {
        let _ = unsafe { Box::from_raw(vm_ptr) };
    }
}
