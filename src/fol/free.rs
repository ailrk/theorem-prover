use crate::fol::ast::*;
use std::collections::HashSet;


impl Term {
    pub fn free_vars(&self) -> HashSet<Var> {
        let mut vars = HashSet::new();
        self.collect_free_vars(&mut vars);
        vars
    }

    fn collect_free_vars(&self, vars: &mut HashSet<Var>) {
        match self {
            Term::Var(var) => {
                vars.insert(var.clone());
            },
            Term::Func(func) => {
                for term in &func.terms {
                    term.collect_free_vars(vars);
                }
            },
            Term::Dummy => {}
        }
    }
}


impl<S> Formula<S> {
    pub fn free_vars(&self) -> HashSet<Var> {
        let mut vars = HashSet::new();
        self.collect_free_vars(&mut vars);
        vars
    }

    fn collect_free_vars(&self, vars: &mut HashSet<Var>) {
        match self {
            Formula::Pred(pred) => {
                for term in &pred.terms {
                    vars.extend(term.free_vars())
                }
            },
            Formula::Not(not) => {
                vars.extend(not.formula.free_vars())
            },
            Formula::And(and) => {
                vars.extend(and.formula1.free_vars());
                vars.extend(and.formula2.free_vars());
            },
            Formula::Or(or) => {
                vars.extend(or.formula1.free_vars());
                vars.extend(or.formula2.free_vars());
            },
            Formula::Implies(imp) => {
                vars.extend(imp.formula1.free_vars());
                vars.extend(imp.formula2.free_vars());
            },
            Formula::Iff(iff) => {
                vars.extend(iff.formula1.free_vars());
                vars.extend(iff.formula2.free_vars());
            },
            Formula::ForAll(forall) => {
                let mut inner = forall.formula.free_vars();
                inner.remove(&forall.var);
                vars.extend(inner)
            },
            Formula::Exists(exists) => {
                let mut inner = exists.formula.free_vars();
                inner.remove(&exists.var);
                vars.extend(inner)
            },
            Formula::Dummy => {}
        }
    }

}
