

#[derive(Debug)]
enum StackErrors {
    PopError,
}
#[allow(dead_code)]
fn push_operand<T>(stack: &mut Vec<T>, operand: T) {
    stack.push(operand);
}
#[allow(dead_code)]
fn pop_operand<T>(stack: &mut Vec<T>) -> Result<T, StackErrors> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err(StackErrors::PopError),
    }
}

#[cfg(test)]
mod tests {
    use crate::stacks::{pop_operand, push_operand};
    #[test]
    fn push_operand_works() {
        let check_stack = vec![1.0, 2.0, 3.0];
        let mut stack: Vec<f64> = Vec::new();
        push_operand(&mut stack, 1.0);
        push_operand(&mut stack, 2.0);
        push_operand(&mut stack, 3.0);
        assert_eq!(stack, check_stack);
    }
    #[test]
    fn pop_operand_works() {
        let check_stack = vec![1.0, 2.0, 3.0];
        let mut stack: Vec<f64> = Vec::new();
        push_operand(&mut stack, 1.0);
        push_operand(&mut stack, 2.0);
        push_operand(&mut stack, 3.0);
        push_operand(&mut stack, 4.0);
        let res = pop_operand(&mut stack);
        assert_eq!(stack, check_stack);
        assert_eq!(res.unwrap(), 4.0);
    }
}
