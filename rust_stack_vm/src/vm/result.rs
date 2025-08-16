#[repr(C)]
pub enum VMResultTag {
    Integer,
    Float,
    Error,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VMResultValue {
    pub int_val: i32,
    pub float_val: f32,
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

    pub fn ok_int(value: i32) -> Self {
        Self {
            tag: VMResultTag::Integer,
            value: VMResultValue {
                int_val: value
            },
        }
    }

    pub fn ok_float(value: f32) -> Self {
        Self {
            tag: VMResultTag::Float,
            value: VMResultValue {
                float_val: value
            },
        }
    }

    pub fn err(value: i32) -> Self {
        Self {
            tag: VMResultTag::Error,
            value: VMResultValue {
                int_val: value
            }
        }
    }
}
