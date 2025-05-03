use crate::fol::ast::*;
use std::collections::HashSet;


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
#[derive(Debug)]
pub struct Clauses(pub Vec<HashSet<Literal>>);


impl Clauses {
    pub fn from_formula(formla: Formula<Cnf>) -> Self {
        let mut result = Vec::new();
        collect_clauses(formla, &mut result);
        Clauses(result)
    }
}


fn collect_clauses(formula: Formula<Cnf>, clauses: &mut Vec<HashSet<Literal>>) {

    fn collect_branch(branch: Formula<Cnf>, clauses: &mut Vec<HashSet<Literal>>) {
        match branch {
            Formula::And(_) => {
                collect_clauses(branch, clauses);
            },
            Formula::Pred(_) | Formula::Or(_) | Formula::Not(_) => {
                let mut clause = HashSet::new();
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
            let mut clause = HashSet::new();
            collect_disjunctives(formula, &mut clause);
            clauses.push(clause);
        },
        _ => panic!("Expect CNF, got {:?}", formula)
    }
}


fn collect_disjunctives(formula: Formula<Cnf>, set: &mut HashSet<Literal>) {

    fn collect_branch(branch: Formula<Cnf>, set: &mut HashSet<Literal>) {
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
