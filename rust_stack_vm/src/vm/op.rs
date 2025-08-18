#[derive(Debug, Clone)]
pub enum OpCode {
    PUSHINT(i32),
    PUSHFLOAT(f32),
    PUSHBYTE(u8),
    PACK(u32),
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    GT,
    GTE,
    LT,
    LTE,
    HALT,
    CONCAT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union OperationValue {
    pub int_val: i32,
    pub uint_val: u32,
    pub float_val: f32,
    pub byte_val: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Operation {
    pub kind: u8,
    pub val: OperationValue,
}
