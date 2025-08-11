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
pub struct COpResult {
    result: c_int,
    is_error: bool,
}

impl COpResult {
    pub fn new(result: c_int, is_error: bool) -> COpResult {
        COpResult {
            result,
            is_error,
        }
    }
    
    pub fn ok(result_code: c_int) -> COpResult {
        COpResult::new(result_code, false)
    }
    
    pub fn err(result_code: c_int) -> COpResult {
        COpResult::new(result_code, true)
    }
}

