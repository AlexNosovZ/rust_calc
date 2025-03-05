#[allow(dead_code)]
fn push_operand<T>(stack: &mut Vec<T>, operand: T) {
    stack.push(operand);
}

#[cfg(test)]
mod tests {
    use crate::stacks::push_operand;
    #[test]
    fn push_operand_works() {
        let check_stack = vec![1.0, 2.0, 3.0];
        let mut stack: Vec<f64> = Vec::new();
        push_operand(&mut stack, 1.0);
        push_operand(&mut stack, 2.0);
        push_operand(&mut stack, 3.0);
        assert_eq!(stack, check_stack);
    }
}
