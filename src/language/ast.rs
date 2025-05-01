use std::collections::HashSet;
use std::fmt::{self};
use std::hash::Hash;


#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Formula {
    Pred(Pred),
    Not(Not),
    And(And),
    Or(Or),
    Implies(Implies),
    ForAll(ForAll),
    Exists(Exists),
    Dummy,
}


#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Term {
    Var(Var),
    Func(Func),
    Dummy,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Var {
    pub name: String,
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
    pub formula1: Box<Formula>,
    pub formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Implies {
    pub formula1: Box<Formula>,
    pub formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ForAll {
    pub var: Var,
    pub formula: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Exists {
    pub var: Var,
    pub formula: Box<Formula>
}


impl Var {
    pub fn from_string(name: String) -> Self { Var{ name }}
    pub fn to_term(&self) -> Term { Term::Var(self.clone()) }
}


impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}


impl Term {
    pub fn var(name: &str) -> Term {
        Term::Var(Var { name: name.to_string() })
    }

    pub fn func(name: &str, terms: Vec<Term>) -> Term {
        Term::Func(Func { name: name.to_string(), terms })
    }
}


impl Default for Term {
    fn default() -> Self {
        Term::Dummy
    }
}


impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0) // Start with indentation level 0
    }
}


impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.dislay(f)
    }
}


impl Formula {
    pub fn pred(name: &str, terms: Vec<Term>) -> Formula {
        Formula::Pred(Pred {
            name: name.to_string(),
            terms
        })
    }

    pub fn implies(formula1: Formula, formula2: Formula) -> Formula {
        Formula::Implies(Implies {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        })
    }

    pub fn or(formula1: Formula, formula2: Formula) -> Formula {
        Formula::Or(Or {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        })
    }

    pub fn and(formula1: Formula, formula2: Formula) -> Formula {
        Formula::And(And {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        })
    }

    pub fn not(formula: Formula) -> Formula {
        Formula::Not(Not {
            formula: Box::new(formula)
        })
    }

    pub fn forall(var: Var, formula: Formula) -> Formula {
        Formula::ForAll(ForAll {
            var,
            formula: Box::new(formula)
        })
    }

    pub fn exists(var: Var, formula: Formula) -> Formula {
        Formula::Exists(Exists {
            var,
            formula: Box::new(formula)
        })
    }
}


impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0) // Start with indentation level 0
    }
}


impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}


impl Default for Formula {
    fn default() -> Self {
        Formula::Dummy
    }
}


fn into_unions(sets: Vec<HashSet<Term>>) -> HashSet<Term> {
    sets.into_iter().fold(HashSet::new(), |b, a| -> HashSet<Term> {
        a.into_iter().chain(b.into_iter()).collect()
    })
}
