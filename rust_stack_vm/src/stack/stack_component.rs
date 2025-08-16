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
    fn pop_n(&mut self, n: usize) -> Result<Vec<T>, StackError>;
}

pub trait ChunkStack<T> {
    fn push_chunk(&mut self, value: Box<[T]>) -> Result<(), StackError>;
    fn pop_chunk(&mut self) -> Result<Box<[T]>, StackError>;
    fn pop_n_chunk(&mut self, n: usize) -> Result<Vec<Box<[T]>>, StackError>;
}

pub struct StackComponent<T> {
    data: Vec<T>,
    chunk_data: Vec<Box<[T]>>,
}

impl<T> StackComponent<T> {
    pub fn new(size: usize) -> Self {
        StackComponent {
            data: Vec::with_capacity(size),
            chunk_data: Vec::with_capacity(size),
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

        if !self.chunk_data.is_empty() {
            debug_struct.field("chunk_data", &self.chunk_data);
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

    fn pop_n(&mut self, n: usize) -> Result<Vec<T>, StackError> {
        if self.data.len() < n {
            return Err(StackError::StackUnderFlow)
        }

        let mut chunks: Vec<T> = self.data.drain(self.data.len() - n..).collect();
        chunks.reverse();

        Ok(chunks)
    }
}

impl<T> ChunkStack<T> for StackComponent<T> {
    fn push_chunk(&mut self, value: Box<[T]>) -> Result<(), StackError> {
        if self.chunk_data.len() == self.chunk_data.capacity() {
            return Err(StackError::StackOverFlow);
        }

        self.chunk_data.push(value);

        Ok(())
    }

    fn pop_chunk(&mut self) -> Result<Box<[T]>, StackError> {
        match self.chunk_data.pop() {
            Some(v) => Ok(v),
            None => Err(StackError::StackUnderFlow),
        }
    }

    fn pop_n_chunk(&mut self, n: usize) -> Result<Vec<Box<[T]>>, StackError> {
        if self.data.len() < n {
            return Err(StackError::StackUnderFlow)
        }

        let mut chunks: Vec<Box<[T]>> = self.chunk_data.drain(self.data.len() - n..).collect();
        chunks.reverse();

        Ok(chunks)
    }
}
