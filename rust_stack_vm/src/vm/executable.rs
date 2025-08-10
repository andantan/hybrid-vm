use std::ffi::c_int;
use crate::stack::stack_component::StackError;

pub trait Executable {
    fn run(&mut self) -> Result<c_int, StackError>;
}