use std::collections::HashSet;
use std::hash::Hash;


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Term {
    Var(Box<Var>),
    UTerm(Box<UTerm>),
    Func(Box<Func>),
    Pred(Box<Pred>),
    Not(Box<Not>),
    And(Box<And>),
    Or(Box<Or>),
    Implies(Box<Implies>),
    ForAll(Box<ForAll>),
    Exists(Box<Exists>),
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Var {
    name: String,
    time: u32
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct UTerm {
    name: String,
    time: u32
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Func {
    name: String,
    terms: Vec<Term>,
    time: u32
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Pred {
    name: String,
    terms: Vec<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Not {
    term: Box<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct And {
    term1: Box<Term>,
    term2: Box<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Or {
    term1: Box<Term>,
    term2: Box<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Implies {
    term1: Box<Term>,
    term2: Box<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ForAll {
    var: Box<Term>,
    term: Box<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Exists {
    var: Box<Term>,
    term: Box<Term>
}


impl Term {
    pub fn free_variables(&self) -> HashSet<Term> {
        match self {
            Term::Var(_) => HashSet::from([self.clone()]),
            Term::UTerm(_) => HashSet::from([self.clone()]),
            Term::Func(func) => {
                if func.terms.len() == 0 {
                    return HashSet::new();
                } else {
                    let fvars = func
                        .terms
                        .iter()
                        .map(|term| term.free_variables())
                        .collect();
                    into_unions(fvars)
                }
            },
            Term::Pred(pred) => {
                if pred.terms.len() == 0 {
                    return HashSet::new();
                } else {
                    let fvars = pred
                        .terms
                        .iter()
                        .map(|term| term.free_variables())
                        .collect();
                    into_unions(fvars)
                }
            },
            Term::Not(not) => not.term.free_variables(),
            Term::And(and) => into_unions(vec![and.term1.free_variables(), and.term2.free_variables()]) ,
            Term::Or(or) => into_unions(vec![or.term1.free_variables(), or.term2.free_variables()]),
            Term::Implies(imp) => into_unions(vec![imp.term1.free_variables(), imp.term2.free_variables()]),
            Term::ForAll(forall) => {
                forall
                    .term
                    .free_variables()
                    .difference(&HashSet::from([*forall.var.clone()]))
                    .cloned()
                    .collect()
            },
            Term::Exists(exists) => {
                exists
                    .term
                    .free_variables()
                    .difference(&HashSet::from([*exists.var.clone()]))
                    .cloned()
                    .collect()
            }
        }
    }

    pub fn free_unification_terms(&self) -> HashSet<Term> {
        match self {
            Term::Var(_) => HashSet::new(),
            Term::UTerm(_) => HashSet::new(),
            Term::Func(func) => {
                if func.terms.len() == 0 {
                    return HashSet::new();
                } else {
                    let fvars = func
                        .terms
                        .iter()
                        .map(|term| term.free_unification_terms())
                        .collect();
                    into_unions(fvars)
                }
            },
            Term::Pred(pred) => {
                if pred.terms.len() == 0 {
                    return HashSet::new();
                } else {
                    let fvars = pred
                        .terms
                        .iter()
                        .map(|term| term.free_unification_terms())
                        .collect();
                    into_unions(fvars)
                }
            },
            Term::Not(not) => not.term.free_unification_terms(),
            Term::And(and) => into_unions(vec![and.term1.free_unification_terms(), and.term2.free_unification_terms()]),
            Term::Or(or) => into_unions(vec![or.term1.free_unification_terms(), or.term2.free_unification_terms()]),
            Term::Implies(imp) => into_unions(vec![imp.term1.free_unification_terms(), imp.term2.free_unification_terms()]),
            Term::ForAll(forall) => forall.term.free_unification_terms(),
            Term::Exists(exists) => exists.term.free_unification_terms()
        }
    }

    pub fn replace(&self, old: Self, new: Self) -> Self {
        match self {
            Term::Var(_) => if old == *self { new } else { self.clone() },
            Term::UTerm(_) => if old == *self { new } else { self.clone() },
            Term::Func(func) =>
                if old == *self { new }
                else {
                    let func1 = Func {
                        name: func.name.clone(),
                        terms: func
                            .terms
                            .iter()
                            .map(|term| term.replace(old.clone(), new.clone()))
                            .collect(),
                        time: 0,
                    };
                    Term::Func(Box::new(func1))
                },
            Term::Pred(pred) =>
                if old == *self { new }
                else {
                    let pred1 = Pred {
                        name: pred.name.clone(),
                        terms: pred
                            .terms
                            .iter()
                            .map(|term| term.replace(old.clone(), new.clone()))
                            .collect(),
                    };
                    Term::Pred(Box::new(pred1))
                },
            Term::Not(not) => Term::Not(Box::new(Not { term: Box::new(not.term.replace(old, new)) })),
            Term::And(and) => Term::And(Box::new(And {
                term1: Box::new(and.term1.replace(old.clone(), new.clone())),
                term2: Box::new(and.term2.replace(old.clone(), new.clone()))
            })),
            Term::Or(or) => Term::Or(Box::new(Or {
                term1: Box::new(or.term1.replace(old.clone(), new.clone())),
                term2: Box::new(or.term2.replace(old.clone(), new.clone()))
            })),
            Term::Implies(imp) => Term::Implies(Box::new(Implies {
                term1: Box::new(imp.term1.replace(old.clone(), new.clone())),
                term2: Box::new(imp.term2.replace(old.clone(), new.clone()))
            })),
            Term::ForAll(forall) => Term::ForAll(Box::new(ForAll {
                var: Box::new(forall.var.replace(old.clone(), new.clone())),
                term: Box::new(forall.term.replace(old.clone(), new.clone()))
            })),
            Term::Exists(exists) => Term::Exists(Box::new(Exists {
                var: Box::new(exists.var.replace(old.clone(), new.clone())),
                term: Box::new(exists.term.replace(old.clone(), new.clone()))
            }))
        }
    }

    pub fn occurs(&self, uterm: &Term) -> bool {
        match self {
            Term::Var(_) => false,
            Term::UTerm(_) => *self == *uterm,
            Term::Func(func) => func.terms.iter().any(|term| term.occurs(uterm)),
            Term::Pred(pred) => pred.terms.iter().any(|term| term.occurs(uterm)),
            Term::Not(not) => not.term.occurs(uterm),
            Term::And(and) => and.term1.occurs(uterm) || and.term2.occurs(uterm),
            Term::Or(or) => or.term1.occurs(uterm) || or.term2.occurs(uterm),
            Term::Implies(imp) => imp.term1.occurs(uterm) || imp.term2.occurs(uterm),
            Term::ForAll(forall) => forall.term.occurs(uterm),
            Term::Exists(exists) => exists.term.occurs(uterm)
        }
    }

    pub fn set_time(&mut self, time: u32) {
        match self {
            Term::Var(var) => var.time = time,
            Term::UTerm(uterm) => uterm.time = time,
            Term::Func(func) => {
                func.time = time;
                for term in &mut func.terms {
                    term.set_time(time)
                }
            },
            Term::Pred(pred) => {
                for term in &mut pred.terms {
                    term.set_time(time)
                }
            },
            Term::Not(not) => {
                not.term.set_time(time)
            },
            Term::And(and) => {
                and.term1.set_time(time);
                and.term2.set_time(time);
            },
            Term::Or(or) => {
                or.term1.set_time(time);
                or.term2.set_time(time);
            },
            Term::Implies(imp) => {
                imp.term1.set_time(time);
                imp.term2.set_time(time);
            },
            Term::ForAll(forall) => {
                forall.var.set_time(time);
                forall.term.set_time(time);
            },
            Term::Exists(exists) => {
                exists.var.set_time(time);
                exists.term.set_time(time);
            }
        }
    }
}


fn into_unions(sets: Vec<HashSet<Term>>) -> HashSet<Term> {
    sets.into_iter().fold(HashSet::new(), |b, a| -> HashSet<Term> {
        a.into_iter().chain(b.into_iter()).collect()
    })
}
