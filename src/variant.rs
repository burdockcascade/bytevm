use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Variant {

    // Null is a null value
    Null,

    // Integer is a 64-bit signed integer
    Integer(i64),

    // Float is a floating point value
    Float(f64),

    // String is a string value
    String(String),

    // Boolean is a boolean value
    Boolean(bool),

    // SymbolReference is a reference to an entry in the global symbol table
    SymbolReference(String),

    // Array is a vector of Variants
    Array(Rc<RefCell<Vec<Variant>>>),

    // Dictionary is a map of Variants
    Dictionary(Rc<RefCell<HashMap<Variant, Variant>>>),
    
    // Index is an index into an array
    Index(usize),

}

impl Variant {

    pub fn pow(&self, rhs: &Variant) -> Variant {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs.pow(*rhs as u32)),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs.powf(*rhs)),
            _ => panic!("Invalid operands for exponentiation")
        }
    }

    pub fn is_false(&self) -> bool {
        match self {
            Variant::Null => true,
            Variant::Boolean(b) => !b,
            Variant::Integer(i) => *i == 0,
            Variant::Float(f) => *f == 0.0,
            Variant::String(s) => s.is_empty(),
            _ => false
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Null => write!(f, "null"),
            Variant::Integer(i) => write!(f, "{}", i),
            Variant::Float(fl) => write!(f, "{}", fl),
            Variant::String(s) => write!(f, "{}", s),
            Variant::Boolean(b) => write!(f, "{}", b),
            Variant::Index(i) => write!(f, "Index({})", i),
            Variant::Array(a) => {
                let a = a.borrow();
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            },
            Variant::Dictionary(d) => {
                let d = d.borrow();
                write!(f, "{{")?;
                for (i, (k, v)) in d.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Variant::SymbolReference(s) => write!(f, "GlobalReference({})", s),
        }
    }
}

impl Into<i64> for Variant {
    fn into(self) -> i64 {
        match self {
            Variant::Integer(i) => i,
            v => panic!("Cannot convert from {:?} to i64", v)
        }
    }
}

impl Into<f64> for Variant {
    fn into(self) -> f64 {
        match self {
            Variant::Float(f) => f,
            v => panic!("Cannot convert from {:?} to f64", v)
        }
    }
}

impl Into<usize> for Variant {
    fn into(self) -> usize {
        match self {
            Variant::Integer(i) => i as usize,
            v => panic!("Cannot convert from {:?} to usize", v)
        }
    }
}

impl Into<String> for Variant {
    fn into(self) -> String {
        match self {
            Variant::String(s) => s,
            _ => panic!("Cannot convert to String")
        }
    }
}

impl Into<bool> for Variant {
    fn into(self) -> bool {
        match self {
            Variant::Null => false,
            Variant::Boolean(b) => b,
            Variant::Integer(i) => i != 0,
            Variant::Float(f) => f != 0.0,
            Variant::String(s) => !s.is_empty(),
            _ => panic!("Cannot convert to bool")
        }
    }
}


impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Variant::Null, Variant::Null) => true,
            (Variant::Integer(lhs), Variant::Integer(rhs)) => lhs == rhs,
            (Variant::Float(lhs), Variant::Float(rhs)) => lhs == rhs,
            (Variant::String(lhs), Variant::String(rhs)) => lhs == rhs,
            (Variant::Boolean(lhs), Variant::Boolean(rhs)) => lhs == rhs,
            (Variant::Array(lhs), Variant::Array(rhs)) => {
                let lhs = lhs.borrow();
                let rhs = rhs.borrow();
                if lhs.len() != rhs.len() {
                    return false;
                }
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    if l != r {
                        return false;
                    }
                }
                true
            },
            (Variant::Dictionary(lhs), Variant::Dictionary(rhs)) => {
                let lhs = lhs.borrow();
                let rhs = rhs.borrow();
                if lhs.len() != rhs.len() {
                    return false;
                }
                for (k, v) in lhs.iter() {
                    if let Some(v2) = rhs.get(k) {
                        if v != v2 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            },
            _ => false
        }
    }
}

impl Eq for Variant {
    fn assert_receiver_is_total_eq(&self) {
        // no-op
    }
}

impl Hash for Variant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Variant::Null => 0.hash(state),
            Variant::Integer(i) => i.hash(state),
            Variant::Float(f) => f.to_bits().hash(state),
            Variant::String(s) => s.hash(state),
            Variant::Boolean(b) => b.hash(state),
            Variant::Index(i) => i.hash(state),
            Variant::Array(a) => {
                let a = a.borrow();
                a.len().hash(state);
                for v in a.iter() {
                    v.hash(state);
                }
            },
            Variant::Dictionary(d) => {
                let d = d.borrow();
                d.len().hash(state);
                for (k, v) in d.iter() {
                    k.hash(state);
                    v.hash(state);
                }
            }
            Variant::SymbolReference(s) => s.hash(state),
        }
    }
}

// Add Operator trait to Variant
impl Add for Variant {
    type Output = Variant;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs + rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs + rhs),
            (Variant::String(lhs), rhs) => Variant::String(lhs + &rhs.to_string()),
            (Variant::Boolean(lhs), Variant::Boolean(rhs)) => Variant::Boolean(lhs && rhs),
            (Variant::Array(lhs), Variant::Array(rhs)) => {
                let mut lhs = lhs.borrow().clone();
                let rhs = rhs.borrow();
                lhs.extend(rhs.iter().cloned());
                Variant::Array(Rc::new(RefCell::new(lhs)))
            },
            (Variant::Dictionary(lhs), Variant::Dictionary(rhs)) => {
                let mut lhs = lhs.borrow().clone();
                let rhs = rhs.borrow();
                for (k, v) in rhs.iter() {
                    lhs.insert(k.clone(), v.clone());
                }
                Variant::Dictionary(Rc::new(RefCell::new(lhs)))
            },
            _ => panic!("Invalid operands for addition")
        }
    }
}

impl Sub for Variant {
    type Output = Variant;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs - rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs - rhs),
            (a, b) => panic!("Invalid operands for subtraction: {} - {}", a, b)
        }
    }
}

impl Div for Variant {
    type Output = Variant;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs / rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs / rhs),
            _ => panic!("Invalid operands for division")
        }
    }
}

impl Mul for Variant {
    type Output = Variant;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs * rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs * rhs),
            _ => panic!("Invalid operands for multiplication")
        }
    }
}

impl Rem for Variant {
    type Output = Variant;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs % rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs % rhs),
            _ => panic!("Invalid operands for modulus")
        }
    }
}

impl Neg for Variant {
    type Output = Variant;

    fn neg(self) -> Self::Output {
        match self {
            Variant::Integer(i) => Variant::Integer(-i),
            Variant::Float(f) => Variant::Float(-f),
            Variant::Boolean(b) => Variant::Boolean(!b),
            _ => panic!("Invalid operand for negation")
        }
    }
}

impl Not for Variant {
    type Output = Variant;

    fn not(self) -> Self::Output {
        match self {
            Variant::Boolean(b) => Variant::Boolean(!b),
            Variant::Integer(i) => Variant::Boolean(i == 0),
            Variant::Float(f) => Variant::Boolean(f == 0.0),
            Variant::String(s) => Variant::Boolean(s.is_empty()),
            _ => panic!("Invalid operand for not operation")
        }
    }
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => lhs.partial_cmp(rhs),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_less_than_or_equal() {
        assert_eq!(Variant::Integer(1) <= Variant::Integer(1), true);
    }

    #[test]
    fn test_float_less_than_or_equal() {
        assert_eq!(Variant::Float(1.0) <= Variant::Float(1.0), true);
    }

    #[test]
    fn test_integer_greater_than() {
        assert_eq!(Variant::Integer(2) > Variant::Integer(1), true);
    }

    #[test]
    fn test_float_greater_than() {
        assert_eq!(Variant::Float(2.0) > Variant::Float(1.0), true);
    }

    #[test]
    fn test_integer_greater_than_or_equal() {
        assert_eq!(Variant::Integer(2) >= Variant::Integer(2), true);
    }

    #[test]
    fn test_float_greater_than_or_equal() {
        assert_eq!(Variant::Float(2.0) >= Variant::Float(2.0), true);
    }

    #[test]
    fn test_boolean_negation() {
        assert_eq!(!Variant::Boolean(false), Variant::Boolean(true));
    }

    #[test]
    fn test_boolean_not() {
        assert_eq!(Variant::Boolean(false).not(), Variant::Boolean(true));
    }

}