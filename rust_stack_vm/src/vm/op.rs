use std::ffi::c_int;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum OpCode {
    PUSH(c_int),
    ADD,
    HALT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct COpCode {
    pub kind: i32,
    pub val: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct OpResult {
    result_code: c_int,
    is_error: bool,
}

impl OpResult {
    pub fn new(result_code: c_int, is_error: bool) -> OpResult {
        OpResult {
            result_code,
            is_error,
        }
    }
    
    pub fn ok(result_code: c_int) -> OpResult {
        OpResult::new(result_code, false)
    }
    
    pub fn err(result_code: c_int) -> OpResult {
        OpResult::new(result_code, true)
    }
}

