use crate::fol::ast::*;
use crate::sat::dimacs;
use std::{collections::HashSet, iter::FromIterator};
use std:: ops::{Deref, DerefMut};
use std::fmt::Display;
use std::io;
use std::io::BufRead;


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Literal {
    Pos(String),
    Neg(String)
}


/* Literal for set of set representation */
impl Literal {
    pub fn pos(s: String) -> Self { Literal::Pos(s) }
    pub fn neg(s: String) -> Self { Literal::Neg(s) }

    pub fn var_name(&self) -> &str {
        match self {
            Literal::Pos(s) | Literal::Neg(s) => s,
        }
    }

    pub fn is_negated(&self) -> bool {
        matches!(self, Literal::Neg(_))
    }

    pub fn negate(&self) -> Self {
        match self {
            Literal::Pos(s) => Literal::Neg(s.clone()),
            Literal::Neg(s) => Literal::Pos(s.clone()),
        }
    }
}


/* Set of set representation of CNF */
#[derive(Debug, Clone)]
pub struct Clauses(pub Vec<Clause>);


#[derive(Debug, Clone)]
pub struct Clause(pub HashSet<Literal>);


#[derive(Debug, Clone)]
pub struct SATSolver(pub fn(Clauses) -> bool);


impl Deref for Clauses {
    type Target = Vec<Clause>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for Clauses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Deref for Clause {
    type Target = HashSet<Literal>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for Clause {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        for lit in self.0.iter() {
            match lit {
                Literal::Pos(pos) => write!(f, "{} ", pos)?,
                Literal::Neg(neg) => write!(f, "-{} ", neg)?,
            }
        }
        write!(f, "}}")
    }
}


impl Display for Clauses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for clause in self.0.iter() {
            write!(f, "{}", clause)?;
        }
        write!(f, "]")
    }
}


impl FromIterator<Literal> for Clause {
    fn from_iter<I: IntoIterator<Item = Literal>>(iter: I) -> Self {
        Clause(HashSet::from_iter(iter))
    }
}


impl FromIterator<Clause> for Clauses {
    fn from_iter<I: IntoIterator<Item = Clause>>(iter: I) -> Self {
        Clauses(Vec::from_iter(iter))
    }
}


impl Clause {
    pub fn new() -> Self {
        Clause(HashSet::new())
    }

    pub fn remove_trivals(&mut self) {
        let symbols = self.iter().map(|lit| lit.var_name().to_string()).collect::<Vec<_>>();
        for symbol in symbols {
            let pos = Literal::pos(symbol.clone());
            let neg = Literal::neg(symbol);
            if self.contains(&pos) && self.contains(&neg) {
                self.remove(&pos);
                self.remove(&neg);
            }
        }
    }
}


impl Clauses {
    pub fn from_formula(formla: Formula<Cnf>) -> Self {
        let mut result = Clauses::new();
        collect_clauses(formla, &mut result);
        result
    }

    pub fn to_formula(&self) -> Formula<Cnf> {
        fn on_clause(clause: Clause) -> Formula<Cnf> {
            clause.iter().map(on_lit).collect::<Vec<_>>().into_iter().reduce(|l, r| { Formula::or(l, r) }).unwrap()
        }

        fn on_lit(lit: &Literal) -> Formula<Cnf> {
            match lit {
                Literal::Pos(s) => Formula::pred(&s, vec![]),
                Literal::Neg(s) => Formula::not(Formula::pred(&s, vec![])),
            }
        }
        self.iter()
            .cloned()
            .map(on_clause)
            .collect::<Vec<_>>()
            .into_iter()
            .reduce(|l, r| { Formula::and(l, r) })
            .unwrap()
    }

    pub fn from_dimacs<R: BufRead>(reader: R) -> io::Result<Self> {
        dimacs::parse(reader)
    }

    pub fn new() -> Self {
        Clauses(Vec::new())
    }

    pub fn is_satisfiable(self,  sat: SATSolver) -> bool {
        sat.0(self)
    }

    pub fn is_valid(self, sat: SATSolver) -> bool {
        let neg = Formula::not(self.to_formula()).cast::<Raw>().to_nnf().to_pnf().skolemize().ground().to_cnf();
        !sat.0(Clauses::from_formula(neg))
    }
}


fn collect_clauses(formula: Formula<Cnf>, clauses: &mut Clauses) {
    fn collect_branch(branch: Formula<Cnf>, clauses: &mut Clauses) {
        match branch {
            Formula::And(_) => {
                collect_clauses(branch, clauses);
            },
            Formula::Pred(_) | Formula::Or(_) | Formula::Not(_) => {
                let mut clause = Clause::new();
                collect_disjunctives(branch, &mut clause);
                clauses.push(clause);
            },
            _ => panic!("Expect CNF, got {:?}", branch)
        };
    }

    match formula {
        Formula::And(And { formula1, formula2, .. }) => {
            collect_branch(*formula1, clauses);
            collect_branch(*formula2, clauses);
        },
        Formula::Pred(_) | Formula::Or(_) | Formula::Not(_) => {
            let mut clause = Clause::new();
            collect_disjunctives(formula, &mut clause);
            clauses.push(clause);
        },
        _ => panic!("Expect CNF, got {:?}", formula)
    }
}


fn collect_disjunctives(formula: Formula<Cnf>, set: &mut Clause) {
    fn collect_branch(branch: Formula<Cnf>, set: &mut Clause) {
        match branch {
            Formula::Pred(_) => {
                collect_disjunctives(branch, set);
            },
            Formula::Not(_) => {
                collect_disjunctives(branch, set);
            },
            Formula::Or(_) => {
                collect_disjunctives(branch, set);
            }
            _ => panic!("Expect disjunctives, got {:?}", branch)
        };
    }

    match formula {
        Formula::Or(Or { formula1, formula2, .. }) => {
            collect_branch(*formula1, set);
            collect_branch(*formula2, set);
        },
        Formula::Not(not) => {
            if let Formula::Pred(pred) = *not.formula {
                set.insert(Literal::neg(pred.unique()));
            }
        },
        Formula::Pred(pred) => {
            set.insert(Literal::pos(pred.unique()));
        },
        _ => panic!("Expect disjunctives, got {:?}", formula)
    };
}
