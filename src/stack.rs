use crate::variant::Variant;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct StackFrame {
    pub locals: Vec<Variant>,
    pub operands: Vec<Variant>
}

impl StackFrame {

    pub fn pop_operand(&mut self) -> Variant {
        self.operands.pop().expect("Operand stack should not be empty")
    }

    pub fn push_operand(&mut self, operand: Variant) {
        self.operands.push(operand);
    }

    pub fn get_local(&self, index: usize) -> Variant {
        self.locals.get(index).cloned().unwrap_or(Variant::Null)
    }

    pub fn set_local(&mut self, index: usize, value: Variant) {
        if index >= self.locals.len() {
            self.locals.resize(index + 1, Variant::Null);
        }
        self.locals[index] = value;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variant::Variant;

    #[test]
    fn test_stack_frame() {
        let mut frame = StackFrame::default();
        frame.push_operand(Variant::Integer(42));
        frame.push_operand(Variant::Integer(100));
        assert_eq!(frame.pop_operand(), Variant::Integer(100));
        assert_eq!(frame.pop_operand(), Variant::Integer(42));
    }

    #[test]
    #[should_panic]
    fn test_stack_frame_panic() {
        let mut frame = StackFrame::default();
        frame.pop_operand();
    }

    #[test]
    fn test_stack_frame_locals() {
        let mut frame = StackFrame::default();
        frame.set_local(0, Variant::Integer(42));
        frame.set_local(1, Variant::Integer(100));
        assert_eq!(frame.get_local(0), Variant::Integer(42));
        assert_eq!(frame.get_local(1), Variant::Integer(100));
    }

    #[test]
    fn test_stack_frame_resize() {
        let mut frame = StackFrame::default();
        frame.set_local(2, Variant::Integer(42));
        assert_eq!(frame.get_local(0), Variant::Null);
        assert_eq!(frame.get_local(1), Variant::Null);
        assert_eq!(frame.get_local(2), Variant::Integer(42));
    }
}