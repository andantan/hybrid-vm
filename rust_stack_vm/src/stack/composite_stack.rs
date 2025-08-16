use crate::stack::stack::StackComponent;

#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    Integer(i32),
    Float(f32),
    Byte(u8),
    // ByteArray is container that include image, data, address,
    // hash, Non UTF-8 encoding (EUC-KR) etc...
    ByteArray(Vec<u8>),
}

pub type CompositeStack = StackComponent<StackValue>;

#[cfg(test)]
mod tests {
    use crate::stack::composite_stack::{CompositeStack};
    use crate::stack::composite_stack::StackValue::Integer;
    use crate::stack::stack::{Stack, StackError};

    #[test]
    fn test_composite_byte_stack_basic() {
        let mut stack = CompositeStack::new(10);

        for i in 1..=10 {
            assert_eq!(stack.push(Integer(i)), Ok(()));
        }

        for i in (1..=10).rev() {
            assert_eq!(stack.pop(), Ok(Integer(i)));
        }
    }

    #[test]
    fn test_composite_byte_stack_underflow() {
        let mut stack = CompositeStack::new(5);

        assert_eq!(stack.push(Integer(10)), Ok(()));
        assert_eq!(stack.pop().unwrap(), Integer(10));
        assert_eq!(stack.pop(), Err(StackError::StackUnderFlow));
    }

    #[test]
    fn test_composite_byte_stack_overflow() {
        let mut stack = CompositeStack::new(2);

        assert_eq!(stack.push(Integer(10)), Ok(()));
        assert_eq!(stack.push(Integer(10)), Ok(()));
        assert_eq!(stack.push(Integer(10)), Err(StackError::StackOverFlow));
    }

    #[test]
    fn test_composite_byte_stack_pop_n() {
        let mut stack = CompositeStack::new(10);

        for i in 1..=10 {
            assert_eq!(stack.push(Integer(i)), Ok(()));
        }

        let popped = stack.pop_n(3);
        assert_eq!(popped, Ok(vec![
            Integer(10),
            Integer(9),
            Integer(8)
        ]));
        assert_eq!(stack.pop(), Ok(Integer(7)));

        let mut stack_failure = CompositeStack::new(10);
        stack_failure.push(Integer(10)).unwrap();

        let popped_failure = stack_failure.pop_n(3);
        assert_eq!(popped_failure, Err(StackError::StackUnderFlow));
    }
}