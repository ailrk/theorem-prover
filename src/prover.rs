use crate::language::*;
use std::collections::{HashSet, HashMap};
use std::fmt::{self};


#[derive(Eq, PartialEq, Clone, Debug)]
struct Unifier(Vec<HashMap<Term, Term>>);


impl Unifier {
    fn new() -> Self {
        Unifier (vec![])
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn cascade(&mut self, other: &mut Unifier) {
        self.0.append(&mut other.0)
    }
}


impl Formula {
    fn to_pnf(&mut self) -> &mut Formula {
        todo!()
    }

    fn to_cnf(&mut self) -> &mut Formula {
        todo!()
    }

    fn skolemize(&mut self) -> &mut Formula {
        todo!()
    }

    fn to_casual_form(&mut self) -> &mut Formula {
        todo!()
    }

    fn unify(&mut self) -> Unifier {
        todo!()
    }

    fn resolve(&mut self) {
        todo!()
    }
}


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
