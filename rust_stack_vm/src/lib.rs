mod vm;
mod stack;

use vm::executable::Executable;
use vm::vm::VM;
use vm::op::{OpCode, OpResult};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_vm() -> *mut VM {
    let instructions = vec![
        OpCode::PUSH(-20000),
        OpCode::PUSH(30000),
        OpCode::ADD,
        // Opcode::HALT,
    ];

    let vm = VM::new(instructions);

    Box::into_raw(Box::new(vm))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_vm(vm_ptr: *mut VM) -> OpResult {
    let vm = unsafe {
        &mut *vm_ptr
    };

    match vm.run() {
        Ok(v) => OpResult::ok(v),
        Err(_) => OpResult::err(-1)
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
