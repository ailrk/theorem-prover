use crate::language::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};


struct Substitution(HashMap<Term, Term>);


fn unify(term1: Term, term2: Term) -> Option<Substitution> {
    match term1 {
        Term::Var(var) => todo!(),
        Term::UTerm(uterm) => todo!(),
        Term::Func(func) => todo!(),
        Term::Pred(pred) => todo!(),
        Term::Not(not) => todo!(),
        Term::And(and) => todo!(),
        Term::Or(or) => todo!(),
        Term::Implies(imp) => todo!(),
        Term::ForAll(forall) => todo!(),
        Term::Exists(exists) => todo!(),
    }
}


fn unify_list(pairs: &Vec<(Term, Term)>) -> Option<Substitution> {
    todo!()
}

struct Result(HashSet<Term>);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Axioms(HashMap<Term, u32>);

impl Hash for Axioms {
    fn hash<H: Hasher>(&self, state: &mut H) { hash_hashmap(&self.0, state) }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Formulas(HashMap<Term, u32>);

impl Hash for Formulas {
    fn hash<H: Hasher>(&self, state: &mut H) { hash_hashmap(&self.0, state) }
}


fn hash_hashmap<H: Hasher>(m: &HashMap<Term, u32>, state: &mut H) {
    for (k, v) in m.iter() {
        k.hash(state);
        v.hash(state);
    }
}


#[derive(PartialEq, Eq, Clone, Debug)]
struct Sequent {
    left: Axioms,
    right: Formulas,
    silblings: Option<HashSet<Sequent>>,
    depth: u32
}


impl Hash for Sequent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
        self.depth.hash(state);
        if let Some(set) = &self.silblings {
            for v in set.iter() {
                v.hash(state);
            }
        }
    }
}


impl Sequent {
    fn free_variables(&self) -> Result {
        todo!()
    }

    fn free_unification_terms(&self) -> Result {
        todo!()
    }

    fn get_variable_name(&self, prefix: &str) -> String {
        todo!()
    }

    fn get_unifiable_pairs(&self) -> Vec<(Term, Term)> {
        todo!()
    }
}


fn prove_sequent(seq: Sequent) -> bool {
    true
}


fn prove_term(axioms: &Axioms, formulas: &Formulas) {
    todo!()
}
