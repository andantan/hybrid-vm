use crate::stack::stack_component::StackComponent;

pub type CompositeStackI32 = StackComponent<i32>;

#[cfg(test)]
mod tests {
    use crate::stack::composite_stack::CompositeStackI32;
    use crate::stack::stack_component::{Stack, StackError};

    #[test]
    fn test_composite_byte_stack_basic() {
        let mut stack = CompositeStackI32::new(10);

        for i in 1..=10 {
            assert_eq!(stack.push(i), Ok(()));
        }

        for i in (1..=10).rev() {
            assert_eq!(stack.pop(), Ok(i));
        }
    }

    #[test]
    fn test_composite_byte_stack_underflow() {
        let mut stack = CompositeStackI32::new(5);

        assert_eq!(stack.push(10), Ok(()));
        assert_eq!(stack.pop().unwrap(), 10);
        assert_eq!(stack.pop(), Err(StackError::StackUnderFlow));
    }

    #[test]
    fn test_composite_byte_stack_overflow() {
        let mut stack = CompositeStackI32::new(2);

        assert_eq!(stack.push(10), Ok(()));
        assert_eq!(stack.push(20), Ok(()));
        assert_eq!(stack.push(30), Err(StackError::StackOverFlow));
    }

    #[test]
    fn test_composite_byte_stack_pop_n() {
        let mut stack = CompositeStackI32::new(10);

        for i in 1..=10 {
            assert_eq!(stack.push(i), Ok(()));
        }

        let popped = stack.pop_n(3);
        assert_eq!(popped, Ok(vec![10, 9, 8]));
        assert_eq!(stack.pop(), Ok(7));

        let mut stack_failure = CompositeStackI32::new(10);
        stack_failure.push(10).unwrap();

        let popped_failure = stack_failure.pop_n(3);
        assert_eq!(popped_failure, Err(StackError::StackUnderFlow));
    }
}