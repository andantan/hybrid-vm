use std::ffi::{c_float, c_int, c_uchar};

#[repr(C)]
#[derive(Debug, Clone)]
pub enum OpCode {
    PUSHINT(c_int),
    PUSHFLOAT(c_float),
    PUSHBYTE(c_uchar),
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    LT,
    LTE,
    GT,
    GTE,
    HALT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union OperationValue {
    pub int_val: c_int,
    pub float_val: c_float,
    pub byte_val: c_uchar,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Operation {
    pub kind: u8,
    pub val: OperationValue,
}

#[repr(C)]
pub enum VMResultTag {
    Integer,
    Float,
    Error,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VMResultValue {
    pub int_val: c_int,
    pub float_val: c_float,
}

#[repr(C)]
pub struct VMResult {
    pub tag: VMResultTag,
    pub value: VMResultValue,
}

impl VMResult {
    pub fn new(tag: VMResultTag, value: VMResultValue) -> Self {
        Self { tag, value }
    }

    pub fn ok_int(value: c_int) -> Self {
        Self {
            tag: VMResultTag::Integer,
            value: VMResultValue {
                int_val: value
            },
        }
    }

    pub fn ok_float(value: c_float) -> Self {
        Self {
            tag: VMResultTag::Float,
            value: VMResultValue {
                float_val: value
            },
        }
    }

    pub fn err(value: c_int) -> Self {
        Self {
            tag: VMResultTag::Error,
            value: VMResultValue {
                int_val: value
            }
        }
    }
}
