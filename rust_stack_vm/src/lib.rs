mod vm;
mod stack;

use std::slice;
use vm::executable::Executable;
use vm::vm::VM;
use vm::op::{OpCode, COpCode, COpResult};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_vm(
    instruction_ptr: *const COpCode,
    instruction_len: usize,
) -> *mut VM {
    let instruction_slice = unsafe {
        slice::from_raw_parts(instruction_ptr, instruction_len)
    };
    
    let instructions: Vec<OpCode> = instruction_slice.iter().map(|opcode| {
        match opcode.kind {
            0 => OpCode::PUSH(opcode.val),
            1 => OpCode::ADD,
            2 => OpCode::HALT,
            _ => panic!("Unknown opcode: {}", opcode.kind),
        }
    }).collect();

    let vm = VM::new(instructions);

    Box::into_raw(Box::new(vm))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_vm(vm_ptr: *mut VM) -> COpResult {
    let vm = unsafe {
        &mut *vm_ptr
    };

    match vm.run() {
        Ok(v) => COpResult::ok(v),
        Err(_) => COpResult::err(-1)
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
