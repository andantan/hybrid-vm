use std::fmt::{self, Debug, Formatter};
use std::any::type_name;

#[derive(Debug, PartialEq)]
pub enum StackError {
    StackUnderFlow,
    StackOverFlow,
    StackInvalidType,
    DivisionByZero
}

pub trait Stack<T> {
    fn push(&mut self, value: T) -> Result<(), StackError>;
    fn pop(&mut self) -> Result<T, StackError>;
    fn pop_n(&mut self, n: usize, rev: bool) -> Result<Vec<T>, StackError>;
}

pub trait FrameStack<T> {
    fn push_frame(&mut self, value: Box<[T]>) -> Result<(), StackError>;
    fn pop_frame(&mut self) -> Result<Box<[T]>, StackError>;
    fn pop_n_frame(&mut self, n: usize, rev: bool) -> Result<Vec<Box<[T]>>, StackError>;
}

pub struct StackComponent<T> {
    data: Vec<T>,
    frames: Vec<Box<[T]>>,
}

impl<T> StackComponent<T> {
    pub fn new(size: usize) -> Self {
        StackComponent {
            data: Vec::with_capacity(size),
            frames: Vec::with_capacity(size),
        }
    }
}

impl<T: Debug> Debug for StackComponent<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let type_name = type_name::<T>();
        let struct_name = format!("StackComponent<{}>", type_name);

        let mut debug_struct = f.debug_struct(&struct_name);

        if !self.data.is_empty() {
            debug_struct.field("data", &self.data);
        }

        if !self.frames.is_empty() {
            debug_struct.field("frames", &self.frames);
        }

        debug_struct.finish()
    }
}


impl<T> Stack<T> for StackComponent<T> {
    fn push(&mut self, value: T) -> Result<(), StackError> {
        if self.data.len() == self.data.capacity() {
            return Err(StackError::StackOverFlow);
        }

        self.data.push(value);

        Ok(())
    }

    fn pop(&mut self) -> Result<T, StackError> {
        match self.data.pop() {
            Some(v) => Ok(v),
            None => Err(StackError::StackUnderFlow),
        }
    }

    fn pop_n(&mut self, n: usize, rev: bool) -> Result<Vec<T>, StackError> {
        if self.data.len() < n {
            return Err(StackError::StackUnderFlow)
        }

        let mut chunks: Vec<T> = self.data.drain(self.data.len() - n..).collect();

        if rev {
            chunks.reverse();
        }

        Ok(chunks)
    }
}

impl<T> FrameStack<T> for StackComponent<T> {
    fn push_frame(&mut self, value: Box<[T]>) -> Result<(), StackError> {
        if self.frames.len() == self.frames.capacity() {
            return Err(StackError::StackOverFlow);
        }

        self.frames.push(value);

        Ok(())
    }

    fn pop_frame(&mut self) -> Result<Box<[T]>, StackError> {
        match self.frames.pop() {
            Some(v) => Ok(v),
            None => Err(StackError::StackUnderFlow),
        }
    }

    fn pop_n_frame(&mut self, n: usize, rev: bool) -> Result<Vec<Box<[T]>>, StackError> {
        if self.data.len() < n {
            return Err(StackError::StackUnderFlow)
        }

        let mut chunks: Vec<Box<[T]>> = self.frames.drain(self.data.len() - n..).collect();

        if rev {
            chunks.reverse();
        }

        Ok(chunks)
    }
}
