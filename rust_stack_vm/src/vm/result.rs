use std::ptr;

#[repr(C)]
pub enum VMResultTag {
    Integer,
    Float,
    ByteArray,
    Error,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ByteArrayPtr {
    pub ptr: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VMResultValue {
    pub int_val: i32,
    pub float_val: f32,
    pub bytes_array_val: ByteArrayPtr,
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

    pub fn ok_byte_array(mut vec: Vec<u8>) -> Self {
        vec.shrink_to_fit();

        let (ptr, len, capacity) = (vec.as_mut_ptr(), vec.len(), vec.capacity());

        std::mem::forget(vec);

        let byte_array_ptr = ByteArrayPtr { ptr, len, capacity, };

        Self {
            tag: VMResultTag::ByteArray,
            value: VMResultValue {
                bytes_array_val: byte_array_ptr,
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
