use std::collections::HashSet;
use crate::fol::ast::*;


impl Term {
    pub fn substitute(&mut self, from: Var, to: Term) {
        let taken = self.take();
        match taken {
            Term::Var(ref var) => {
                if *var == from {
                    *self = to
                } else {
                    *self = taken
                }
            },
            Term::Func(mut func) => {
                for term in func.terms.iter_mut() {
                    term.substitute(from.clone(), to.clone())
                }
                *self = Term::func( &func.name, func.terms)
            }
            Term::Dummy => {}
        }
    }
}


impl<S> Formula<S> {
    /* Replace var with term. */
    pub fn substitute(&mut self, from: Var, to: Term) {
        match self {
            Formula::Pred(pred) => {
                for term in pred.terms.iter_mut() {
                    term.substitute(from.clone(), to.clone());
                }
            },
            Formula::Not(not) => not.formula.substitute(from.clone(), to),
            Formula::And(and) => {
                and.formula1.substitute(from.clone(), to.clone());
                and.formula2.substitute(from.clone(), to);
            }
            Formula::Or(or) => {
                or.formula1.substitute(from.clone(), to.clone());
                or.formula2.substitute(from.clone(), to);
            },
            Formula::Implies(imp) => {
                imp.formula1.substitute(from.clone(), to.clone());
                imp.formula2.substitute(from.clone(), to);
            },
            Formula::Iff(iff) => {
                iff.formula1.substitute(from.clone(), to.clone());
                iff.formula2.substitute(from.clone(), to);
            },
            /* We need to perform alpha-renaming on quantifier cases. e.g For a given substitution [from/to]
             * on formula `forall x. M`, if to.free_vars() contains x, we need to rename x to avoid capture.
             * So `forall x. M` -> `forall x1. M[x/x1]`.
             * Then we can perform the original substitution safely.
             */
            Formula::ForAll(_) => {
                let mut taken = self.take();
                if let Formula::ForAll(ref forall) = taken {
                    if to.free_vars().contains(&forall.var) {
                        taken.alpha_rename();
                    }
                }
                if let Formula::ForAll(ref mut forall) = taken {
                    if forall.var == from { // bounded
                    } else {
                        forall.formula.substitute(from, to);
                    }
                }
                *self = (&mut taken).take()
            },
            Formula::Exists(_) => {
                let mut taken = self.take();
                if let Formula::Exists(ref exists) = taken {
                    if to.free_vars().contains(&exists.var) {
                        taken.alpha_rename();
                    }
                }
                if let Formula::Exists(ref mut exists) = taken {
                    if exists.var == from { // bounded
                    } else {
                        exists.formula.substitute(from, to);
                    }
                }
                *self = (&mut taken).take()
            },
            Formula::Dummy => {},
        }
    }

    /* Alpha rename a bound variable to a fresh name */
    pub fn alpha_rename(&mut self) {
        match self {
            Formula::ForAll(ref mut forall) => {
                let var = forall.var.clone();
                let free_vars = forall.formula.free_vars();
                let fresh = fresh_name(&var, &free_vars);
                forall.var = fresh.clone();
                forall.formula.substitute(var, fresh.to_term());
            },
            Formula::Exists(ref mut exists) => {
                let var = exists.var.clone();
                let free_vars = exists.formula.free_vars();
                let fresh = fresh_name(&var, &free_vars);
                exists.var = fresh.clone();
                exists.formula.substitute(var, fresh.to_term());
            },
            _ => {}
        }
    }
}


fn fresh_name(base: &Var, taken: &HashSet<Var>) -> Var {
    for i in 0.. {
        let name = Var::from_string(format!("{}{}", base.to_term(), i));
        if !taken.contains(&name) {
            return name;
        }
    }
    unreachable!()
}

