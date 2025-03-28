use crate::variant::Variant;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub locals: Vec<Variant>,
    pub operands: Vec<Variant>,
    pub base_address: usize,
    pub return_address: Option<usize>,
}

impl Default for StackFrame {
    fn default() -> Self {
        StackFrame {
            locals: Vec::with_capacity(16),
            operands: Vec::with_capacity(16),
            base_address: 0,
            return_address: None,
        }
    }
}

impl StackFrame {

    pub fn pop_operand(&mut self) -> Variant {
        self.operands.pop().expect("Operand stack should not be empty")
    }

    pub fn pop_operands(&mut self, count: usize) -> Vec<Variant> {
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(self.pop_operand());
        }
        result.reverse();
        result
    }

    pub fn push_operand(&mut self, operand: Variant) {
        self.operands.push(operand);
    }

    pub fn push_local(&mut self, value: Variant) {
        self.locals.push(value);
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