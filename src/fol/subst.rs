use std::collections::HashSet;
use crate::fol::ast::*;


impl Term {
    pub fn substitute(&mut self, from: Var, to: &mut Term) {
        let taken = std::mem::take(self);
        match taken {
            Term::Var(ref var) => {
                if *var == from {
                    *self = std::mem::take(to)
                } else {
                    *self = taken
                }
            },
            Term::Func(mut func) => {
                for term in func.terms.iter_mut() {
                    term.substitute(from.clone(), to)
                }
                *self = Term::func( &func.name, func.terms)
            }
            Term::Dummy => {}
        }
    }
}


impl<S> Formula<S> {
    /* Replace var with term. */
    pub fn substitute(&mut self, from: Var, to: &mut Term) {
        match self {
            Formula::Pred(pred) => {
                for term in pred.terms.iter_mut() {
                    term.substitute(from.clone(), to);
                }
            },
            Formula::Not(not) => not.formula.substitute(from.clone(), to),
            Formula::And(and) => {
                and.formula1.substitute(from.clone(), to);
                and.formula2.substitute(from.clone(), to);
            }
            Formula::Or(or) => {
                or.formula1.substitute(from.clone(), to);
                or.formula2.substitute(from.clone(), to);
            },
            Formula::Implies(imp) => {
                imp.formula1.substitute(from.clone(), to);
                imp.formula2.substitute(from.clone(), to);
            },
            Formula::Iff(iff) => {
                iff.formula1.substitute(from.clone(), to);
                iff.formula2.substitute(from.clone(), to);
            },
            /* We need to perform alpha-renaming on quantifier cases. e.g For a given substitution [from/to]
             * on formula `forall x. M`, if to.free_vars() contains x, we need to rename x to avoid capture.
             * So `forall x. M` -> `forall x1. M[x/x1]`.
             * Then we can perform the original substitution safely.
             */
            Formula::ForAll(_) => {
                let mut taken = std::mem::take(self);
                if let Formula::ForAll(ref mut forall) = taken {
                    if forall.var == from { // bounded
                    } else if to.free_vars().contains(&forall.var){
                        let free_vars = self.free_vars();
                        let fresh = fresh_name(&from, &free_vars);
                        forall.var = fresh;
                        forall.formula.substitute(from.clone(), to);
                    } else {
                        forall.formula.substitute(from.clone(), to);
                    }
                }
                *self = std::mem::take(&mut taken)
            },
            Formula::Exists(_) => {
                let mut taken = std::mem::take(self);
                if let Formula::Exists(ref mut exists) = taken {
                    if exists.var == from { // bounded
                    } else if to.free_vars().contains(&exists.var){
                        let free_vars = self.free_vars();
                        let fresh = fresh_name(&from, &free_vars);
                        exists.var = fresh;
                        exists.formula.substitute(from.clone(), to);
                    } else {
                        exists.formula.substitute(from.clone(), to);
                    }
                }
                *self = std::mem::take(&mut taken)
            },
            Formula::Dummy => {},
        }
    }
}


fn fresh_name(base: &Var, taken: &HashSet<Var>) -> Var {
    for i in 0.. {
        let name = Var::from_string(format!("{}_{}", base.to_term(), i));
        if !taken.contains(&name) {
            return name;
        }
    }
    unreachable!()
}

