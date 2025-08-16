mod vm;
mod stack;

use std::ffi::{c_float, c_uchar};
use std::slice;
use vm::vm::VM;
use vm::op::{OpCode, Operation};
use stack::stack_component::StackError;
use crate::stack::composite_stack::StackValue;
use crate::vm::op::VMResult;
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
            0 => OpCode::HALT,
            1 => OpCode::PUSHINT( unsafe { opcode.val.int_val } ),
            2 => OpCode::PUSHFLOAT( unsafe { opcode.val.float_val } ),
            3 => OpCode::PUSHBYTE( unsafe { opcode.val.byte_val } ),
            4 => OpCode::POP,
            5 => OpCode::ADD,
            6 => OpCode::SUB,
            7 => OpCode::MUL,
            8 => OpCode::DIV,
            9 => OpCode::EQ,
            10 => OpCode::LT,
            11 => OpCode::LTE,
            12 => OpCode::GT,
            13 => OpCode::GTE,
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
            _ => VMResult::err(-1)
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
pub unsafe extern "C" fn free_vm(vm_ptr: *mut VM) {
    if !vm_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(vm_ptr);
        }
    }
}
