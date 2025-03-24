use std::collections::HashSet;
use std::hash::Hash;


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Formula {
    Pred(Box<Pred>),
    Not(Box<Not>),
    And(Box<And>),
    Or(Box<Or>),
    Implies(Box<Implies>),
    ForAll(Box<ForAll>),
    Exists(Box<Exists>),
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Term {
    Var(Box<Var>),
    Func(Box<Func>),
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Var {
    pub name: String,
    pub time: u32
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Func {
    pub name: String,
    pub terms: Vec<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Pred {
    pub name: String,
    pub terms: Vec<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Not {
    pub formula: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct And {
    pub formula1: Box<Formula>,
    pub formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Or {
    formula1: Box<Formula>,
    formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Implies {
    pub formula1: Box<Formula>,
    pub formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ForAll {
    pub var: Box<Term>,
    pub formula: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Exists {
    pub var: Box<Term>,
    pub formula: Box<Formula>
}


impl Term {
    pub fn var (name: &str) -> Term {
        Term::Var(Box::new(Var { name: name.to_string(), time: 0 }))
    }

    pub fn func(name: &str, terms: Vec<Term>) -> Term {
        Term::Func(Box::new(Func { name: name.to_string(), terms }))
    }
}



impl Formula {
    pub fn pred(name: &str, terms: Vec<Term>) -> Formula {
        Formula::Pred(Box::new(Pred {
            name: name.to_string(),
            terms
        }))
    }

    pub fn implies(formula1: Formula, formula2: Formula) -> Formula {
        Formula::Implies(Box::new(Implies {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        }))
    }

    pub fn or(formula1: Formula, formula2: Formula) -> Formula {
        Formula::Or(Box::new(Or {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        }))
    }

    pub fn and(formula1: Formula, formula2: Formula) -> Formula {
        Formula::And(Box::new(And {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        }))
    }

    pub fn not(formula: Formula) -> Formula {
        Formula::Not(Box::new(Not {
            formula: Box::new(formula)
        }))
    }

    pub fn forall(var: Term, formula: Formula) -> Formula {
        Formula::ForAll(Box::new(ForAll {
            var: Box::new(var),
            formula: Box::new(formula)
        }))
    }

    pub fn exists(var: Term, formula: Formula) -> Formula {
        Formula::Exists(Box::new(Exists {
            var: Box::new(var),
            formula: Box::new(formula)
        }))
    }
}


fn into_unions(sets: Vec<HashSet<Term>>) -> HashSet<Term> {
    sets.into_iter().fold(HashSet::new(), |b, a| -> HashSet<Term> {
        a.into_iter().chain(b.into_iter()).collect()
    })
}
