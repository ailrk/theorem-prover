use crate::fol::ast::*;
use std::collections::HashMap;


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Unifier(Vec<HashMap<Term, Term>>);


impl Unifier {
    pub fn new() -> Self {
        Unifier (vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn cascade(&mut self, other: &mut Unifier) {
        self.0.append(&mut other.0)
    }
}


fn unify<S>(formula: &mut Formula<S>) -> Unifier {
    todo!()
}
