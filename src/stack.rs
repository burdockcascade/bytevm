use crate::variant::Variant;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub id: usize,
    pub locals: Vec<Variant>,
    pub operands: Vec<Variant>,
    pub return_address: Option<usize>,
}

impl StackFrame {
    pub fn new(id: usize) -> StackFrame {
        StackFrame {
            id,
            locals: Vec::with_capacity(16),
            operands: Vec::with_capacity(16),
            return_address: None,
        }
    }

    pub fn pop_operand(&mut self) -> Variant {
        self.operands.pop().expect("Operand stack should not be empty")
    }

    pub fn push_operand(&mut self, operand: Variant) {
        self.operands.push(operand);
    }

    pub fn push_local(&mut self, value: Variant) {
        self.locals.push(value);
    }

    pub fn get_local(&self, index: usize) -> Variant {
        if index >= self.locals.len() {
            panic!("Local variable not found: {}", index);
        } else {
            self.locals[index].clone()
        }
    }

    pub fn set_local(&mut self, index: usize, value: Variant) {
        if index >= self.locals.len() {
            for _ in self.locals.len()..index {
                self.locals.push(Variant::Null);
            }
            self.locals.resize(index + 1, value);
        } else {
            self.locals[index] = value;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variant::Variant;

    #[test]
    fn test_stack_frame() {
        let mut frame = StackFrame::new(0);
        frame.push_operand(Variant::Integer(42));
        frame.push_operand(Variant::Integer(100));
        assert_eq!(frame.pop_operand(), Variant::Integer(100));
        assert_eq!(frame.pop_operand(), Variant::Integer(42));
    }

    #[test]
    #[should_panic]
    fn test_stack_frame_panic() {
        let mut frame = StackFrame::new(0);
        frame.pop_operand();
    }

    #[test]
    fn test_stack_frame_locals() {
        let mut frame = StackFrame::new(0);
        frame.set_local(0, Variant::Integer(42));
        frame.set_local(1, Variant::Integer(100));
        assert_eq!(frame.get_local(0), Variant::Integer(42));
        assert_eq!(frame.get_local(1), Variant::Integer(100));
    }

    #[test]
    fn test_stack_frame_resize() {
        let mut frame = StackFrame::new(0);
        frame.set_local(2, Variant::Integer(42));
        assert_eq!(frame.get_local(0), Variant::Null);
        assert_eq!(frame.get_local(1), Variant::Null);
        assert_eq!(frame.get_local(2), Variant::Integer(42));
    }
}