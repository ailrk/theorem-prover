use crate::language::*;
use std::fmt::{self};


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Sequent {
    pub left: Vec<Formula>,
    pub right: Vec<Formula>,
    pub depth: usize
}


impl fmt::Display for Sequent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sequent { left, right, .. } => {
                for (idx, param) in left.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if idx < left.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " ⊢ ")?;
                for (idx, param) in right.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if idx < right.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                writeln!(f, "")
            }
        }
    }
}
