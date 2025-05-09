use std::fmt::{self};
use std::hash::Hash;
use std::marker::PhantomData;


#[derive(Debug, Clone)] pub struct Raw;
#[derive(Debug, Clone)] pub struct Nnf;
#[derive(Debug, Clone)] pub struct Pnf;
#[derive(Debug, Clone)] pub struct Skolemized;
#[derive(Debug, Clone)] pub struct Grounded;
#[derive(Debug, Clone)] pub struct Cnf;


#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Formula<S: 'static> {
    Pred(Pred<S>),
    Not(Not<S>),
    And(And<S>),
    Or(Or<S>),
    Implies(Implies<S>),
    Iff(Iff<S>),
    ForAll(ForAll<S>),
    Exists(Exists<S>),
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
pub struct Pred<S> {
    pub name: String,
    pub terms: Vec<Term>,
    _marker: std::marker::PhantomData<S>,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Not<S: 'static> {
    pub formula: Box<Formula<S>>,
    _marker: std::marker::PhantomData<S>,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct And<S: 'static> {
    pub formula1: Box<Formula<S>>,
    pub formula2: Box<Formula<S>>,
    _marker: std::marker::PhantomData<S>,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Or<S: 'static> {
    pub formula1: Box<Formula<S>>,
    pub formula2: Box<Formula<S>>,
    _marker: std::marker::PhantomData<S>,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Implies<S: 'static> {
    pub formula1: Box<Formula<S>>,
    pub formula2: Box<Formula<S>>,
    _marker: std::marker::PhantomData<S>,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Iff<S: 'static> {
    pub formula1: Box<Formula<S>>,
    pub formula2: Box<Formula<S>>,
    _marker: std::marker::PhantomData<S>,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ForAll<S: 'static> {
    pub var: Var,
    pub formula: Box<Formula<S>>,
    _marker: std::marker::PhantomData<S>,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Exists<S: 'static> {
    pub var: Var,
    pub formula: Box<Formula<S>>,
    _marker: std::marker::PhantomData<S>,
}


// Treat grounded predicate as an unique atom
impl<S> Pred<S> {
    pub fn unique(&self) -> String {
        fn term_name(term: &Term) -> String {
            match term {
                Term::Var(v) => v.name.clone(),
                Term::Func(f) => {
                    let inner = f.terms.iter().map(term_name).collect::<Vec<_>>().join("");
                    format!("{}{}", f.name, if inner.is_empty() { "".to_string() } else { format!("_{}", inner) })
                }
                Term::Dummy => "dummy".to_string(),
            }
        }
        let args = self.terms.iter().map(term_name).collect::<Vec<_>>().join("_");
        format!("{}_{}", self.name, args)
    }
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

    pub fn take(&mut self) -> Self {
        std::mem::take(self)
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


impl<S> Formula<S> {
    pub fn pred(name: &str, terms: Vec<Term>) -> Formula<S> {
        Formula::Pred(Pred {
            name: name.to_string(),
            terms,
            _marker: PhantomData
        })
    }

    pub fn implies(formula1: Formula<S>, formula2: Formula<S>) -> Formula<S> {
        Formula::Implies(Implies {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2),
            _marker: PhantomData
        })
    }

    pub fn iff(formula1: Formula<S>, formula2: Formula<S>) -> Formula<S> {
        Formula::Iff(Iff {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2),
            _marker: PhantomData
        })
    }

    pub fn or(formula1: Formula<S>, formula2: Formula<S>) -> Formula<S> {
        Formula::Or(Or {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2),
            _marker: PhantomData
        })
    }

    pub fn and(formula1: Formula<S>, formula2: Formula<S>) -> Formula<S> {
        Formula::And(And {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2),
            _marker: PhantomData
        })
    }

    pub fn not(formula: Formula<S>) -> Formula<S> {
        Formula::Not(Not {
            formula: Box::new(formula),
            _marker: PhantomData
        })
    }

    pub fn forall(var: Var, formula: Formula<S>) -> Formula<S> {
        Formula::ForAll(ForAll {
            var,
            formula: Box::new(formula),
            _marker: PhantomData
        })
    }

    pub fn exists(var: Var, formula: Formula<S>) -> Formula<S> {
        Formula::Exists(Exists {
            var,
            formula: Box::new(formula),
            _marker: PhantomData
        })
    }

    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }

    pub fn cast<T>(self) -> Formula<T> {
        match self {
            Formula::Pred(Pred { name, terms, .. }) => Formula::pred(&name, terms),
            Formula::Not(Not { formula, .. }) => Formula::not((*formula).cast()),
            Formula::And(And { formula1, formula2, .. }) => Formula::and((*formula1).cast(), (*formula2).cast()),
            Formula::Or(Or { formula1, formula2, .. }) => Formula::or((*formula1).cast(), (*formula2).cast()),
            Formula::Implies(Implies { formula1, formula2, .. }) => Formula::implies((*formula1).cast(), (*formula2).cast()),
            Formula::Iff(Iff { formula1, formula2, .. }) => Formula::iff((*formula1).cast(), (*formula2).cast()),
            Formula::ForAll(ForAll { var, formula, .. }) => Formula::forall(var, (*formula).cast()),
            Formula::Exists(Exists { var, formula, .. }) => Formula::exists(var, (*formula).cast()),
            Formula::Dummy => Formula::Dummy,
        }
    }
}


impl<S: 'static> fmt::Debug for Formula<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0) // Start with indentation level 0
    }
}


impl<S: 'static> fmt::Display for Formula<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}


impl<S> Default for Formula<S> {
    fn default() -> Self {
        Formula::Dummy
    }
}
