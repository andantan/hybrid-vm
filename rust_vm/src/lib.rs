mod executable;

use std::ffi::c_int;
use executable::Executable;

#[repr(C)]
#[derive(Debug)]
pub enum Opcode {
    PUSH(c_int),
    ADD,
    HALT,
}

pub struct VM {
    stack: Vec<c_int>,
    instructions: Vec<Opcode>,
    ip: usize
}

impl VM {
    pub fn new(instructions: Vec<Opcode>) -> Self {
        VM {
            stack: Vec::new(),
            instructions,
            ip: 0,
        }
    }
}

impl Executable for VM {
    fn run(&mut self) -> Option<c_int> {
        loop {
            if self.ip >= self.instructions.len() {
                return self.stack.pop();
            }

            let opcode = &self.instructions[self.ip];
            self.ip += 1;

            match opcode {
                Opcode::PUSH(v) => {
                    self.stack.push(*v);
                },
                Opcode::ADD => {
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a + b);
                    } else {
                        return None;
                    }
                },
                Opcode::HALT => {
                    return self.stack.pop();
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_vm() -> *mut VM {
    let instructions = vec![
        Opcode::PUSH(3),
        Opcode::PUSH(4),
        Opcode::ADD,
        Opcode::HALT,
    ];

    let vm = VM::new(instructions);

    Box::into_raw(Box::new(vm))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_vm(vm_ptr: *mut VM) -> c_int {
    let vm = unsafe {
        &mut *vm_ptr
    };

    if let Some(result) = vm.run() {
        result
    } else {
        -1
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
