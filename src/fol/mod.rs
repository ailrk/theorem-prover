pub mod ast;
pub mod parser;
pub mod fmt;
pub mod skolem;
use std::collections::HashSet;
use crate::fol::ast::*;


impl Term {
    pub fn free_vars(&self) -> HashSet<Var> { term_free_vars(self) }
    pub fn substitute(&mut self, from: Var, to: Term) { term_substitute(self, from, to) }
}


impl<S> Formula<S> {
    pub fn free_vars(&self) -> HashSet<Var> { formula_free_vars(self) }
    pub fn substitute(&mut self, from: Var, to: Term) { formula_substitute(self, from, to) }
    pub fn alpha_rename(&mut self) { alpha_rename(self) }
}


impl Formula<Raw> {
    pub fn to_nnf(self) -> Formula<Nnf> { to_nnf(self) }
}


impl Formula<Skolemized> {
    pub fn ground(self) -> Formula<Grounded> {
        ground(self)
    }
}


impl Formula<Grounded> {
    pub fn to_cnf(self) -> Formula<Cnf> { to_cnf(self).cast() }
}


impl Formula<Nnf> {
    pub fn to_pnf(self) -> Formula<Pnf> {
        to_pnf(self)
    }
}


impl Formula<Pnf> {
    pub fn skolemize(self) -> Formula<Skolemized> {
        skolem::skolemize(self)
    }
}


fn ground(mut formula: Formula<Skolemized>) -> Formula<Grounded> {
    loop {
        match formula {
            Formula::ForAll(forall) => formula = *forall.formula,
            Formula::Exists(exists) => formula = *exists.formula,
            _ => break
        };
    }
    formula.cast()
}


fn term_free_vars(term: &Term) -> HashSet<Var> {
    fn collect_free_vars(term: &Term, vars: &mut HashSet<Var>) {
        match term {
            Term::Var(var) => {
                vars.insert(var.clone());
            },
            Term::Func(func) => {
                for term in &func.terms {
                    collect_free_vars(term, vars);
                }
            },
            Term::Dummy => {}
        }
    }

    let mut vars = HashSet::new();
    collect_free_vars(term, &mut vars);
    vars
}


fn formula_free_vars<T>(formula: &Formula<T>) -> HashSet<Var> {
    let mut vars = HashSet::new();
    match formula {
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
    vars
}


fn term_substitute(term: &mut Term, from: Var, to: Term) {
    let taken = term.take();
    match taken {
        Term::Var(ref var) => {
            if *var == from {
                *term = to
            } else {
                *term = taken
            }
        },
        Term::Func(mut func) => {
            for term in func.terms.iter_mut() {
                term.substitute(from.clone(), to.clone())
            }
            *term = Term::func( &func.name, func.terms)
        }
        Term::Dummy => {}
    }
}


fn formula_substitute<T>(formula: &mut Formula<T>, from: Var, to: Term) {
    match formula {
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
        // We need to perform alpha-renaming on quantifier cases. e.g For a given substitution [from/to]
        // on formula `forall x. M`, if to.free_vars() contains x, we need to rename x to avoid capture.
        // So `forall x. M` -> `forall x1. M[x/x1]`.
        // Then we can perform the original substitution safely.
        Formula::ForAll(_) => {
            let mut taken = formula.take();
            if let Formula::ForAll(ref forall) = taken {
                if to.free_vars().contains(&forall.var) {
                    alpha_rename(&mut taken);
                }
            }
            if let Formula::ForAll(ref mut forall) = taken {
                if forall.var == from { // bounded
                } else {
                    forall.formula.substitute(from, to);
                }
            }
            *formula = (&mut taken).take()
        },
        Formula::Exists(_) => {
            let mut taken = formula.take();
            if let Formula::Exists(ref exists) = taken {
                if to.free_vars().contains(&exists.var) {
                    alpha_rename(&mut taken);
                }
            }
            if let Formula::Exists(ref mut exists) = taken {
                if exists.var == from { // bounded
                } else {
                    exists.formula.substitute(from, to);
                }
            }
            *formula = (&mut taken).take()
        },
        Formula::Dummy => {},
    }
}


/* Alpha rename a bound variable to a fresh name */
pub fn alpha_rename<T>(formula: &mut Formula<T>) {
    match formula {
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


fn fresh_name(base: &Var, taken: &HashSet<Var>) -> Var {
    for i in 0.. {
        let name = Var::from_string(format!("{}{}", base.to_term(), i));
        if !taken.contains(&name) {
            return name;
        }
    }
    unreachable!()
}


/* NNF conversion */


fn to_nnf(formula: Formula<Raw>) -> Formula<Nnf> {
     // Eliminate Implications and Biconditionals:
     // (A → B) becomes ¬A ∨ B
     // (A ⇔  B) becomes (A ∧ B) ∨ (¬A ∧ ¬B)
    fn eliminate_arrows(formula: Formula<Raw>) -> Formula<Raw> {
        match formula {
            Formula::Implies(Implies { formula1, formula2, .. }) => Formula::or(Formula::not(eliminate_arrows(*formula1)), eliminate_arrows(*formula2)),
            Formula::Iff(Iff { formula1, formula2, .. }) => {
                let formula1_noarrow = eliminate_arrows(*formula1);
                let formula2_noarrow = eliminate_arrows(*formula2);
                Formula::or(
                    Formula::and(formula1_noarrow.clone(), formula2_noarrow.clone()),
                    Formula::and(Formula::not(formula1_noarrow), Formula::not(formula2_noarrow)))
            },
            Formula::Not(Not { formula, .. }) => Formula::not(eliminate_arrows(*formula)),
            Formula::And(And { formula1, formula2, .. }) => Formula::and(eliminate_arrows(*formula1), eliminate_arrows(*formula2)),
            Formula::Or(Or { formula1, formula2, .. }) => Formula::or(eliminate_arrows(*formula1), eliminate_arrows(*formula2)),
            Formula::ForAll(ForAll { var, formula, .. } ) => Formula::forall(var, eliminate_arrows(*formula)),
            Formula::Exists(Exists { var, formula, .. }) => Formula::exists(var, eliminate_arrows(*formula)),
            Formula::Pred(_) => formula,
            Formula::Dummy => formula,
        }
    }

    //  De Morgan's Laws for connectives:
    //  ¬(A ∧ B) becomes ¬A ∨ ¬B
    //  ¬(A ∨ B) becomes ¬A ∧ ¬B
    //
    //  Quantifier negation (handle negations of quantifiers):
    //  ¬∀x.M becomes ∃x.¬M
    //  ¬∃x.M becomes ∀x.¬M
    fn push_negations(formula: Formula<Raw>, pushed: &mut u32) -> Formula<Raw> {
        match formula {
            Formula::Not(Not { formula, .. }) => {
                match *formula {
                    Formula::And(And { formula1, formula2, .. }) => {
                        *pushed += 1;
                        Formula::or(
                            Formula::not(push_negations(*formula1, pushed)),
                            Formula::not(push_negations(*formula2, pushed)))
                    },
                    Formula::Or(Or { formula1, formula2, .. }) => {
                        *pushed += 1;
                        Formula::and(
                            Formula::not(push_negations(*formula1, pushed)),
                            Formula::not(push_negations(*formula2, pushed)))
                    },
                    Formula::ForAll(ForAll { var, formula, .. }) => {
                        *pushed += 1;
                        Formula::exists(var, Formula::not(push_negations(*formula, pushed)))
                    },
                    Formula::Exists(Exists { var, formula, .. }) => {
                        *pushed += 1;
                        Formula::forall(var, Formula::not(push_negations(*formula, pushed)))
                    }
                    Formula::Not(Not { formula, .. }) => {
                        push_negations(*formula, pushed)
                    }
                    _ => {
                        Formula::not(push_negations(*formula, pushed))
                    }
                }
            },
            Formula::And(And { formula1, formula2, .. }) =>
                Formula::and(push_negations(*formula1, pushed), push_negations(*formula2, pushed)),
            Formula::Or(Or { formula1, formula2, .. }) =>
                Formula::or(push_negations(*formula1, pushed), push_negations(*formula2, pushed)),
            Formula::ForAll(ForAll { var, formula, .. } ) =>
                Formula::forall(var, push_negations(*formula, pushed)),
            Formula::Exists(Exists { var, formula, .. }) =>
                Formula::exists(var, push_negations(*formula, pushed)),
            Formula::Implies(Implies { formula1, formula2, .. }) =>
                Formula::implies(push_negations(*formula1, pushed), push_negations(*formula2, pushed)),
            Formula::Iff(Iff { formula1, formula2, .. }) =>
                Formula::iff(push_negations(*formula1, pushed), push_negations(*formula2, pushed)),
            Formula::Pred(_) => formula,
            Formula::Dummy => formula,
        }
    }

    let mut formula = eliminate_arrows(formula);
    loop {
        let mut pushed = 0;
        let new_formula = push_negations(formula, &mut pushed);
        if pushed == 0 {
            break new_formula.cast()
        }
        formula = new_formula
    }
}


/* CNF Conversion */


/*  p ∧ q => cnf(p) ∧ cnf(q)
 *  p ∨ q => distribute_or(cnf(p) ∨ cnf(q))
 *  Because it's already grounded this covers all cases.
 *  */
fn to_cnf(formula: Formula<Grounded>) -> Formula<Grounded> {
    // p ∨ (q ∧ r) ⇔  (p ∨ q) ∧  (p ∨ r)
    // (p ∧ q) ∨ r ⇔  (p ∨ r) ∧ (q ∨ r).
    fn distribute_or(formula: Formula<Grounded>) -> Formula<Grounded> {
        match formula {
            Formula::Or(Or { formula1: mut o1, formula2: mut o2, .. }) => {
                if let Formula::And(And { formula1: a1, formula2: a2, .. }) = &mut *o2 {
                    let p = Formula::or(*o1.clone(), (&mut *a1).take());
                    let q = Formula::or(*o1, (*a2).take());
                    Formula::and(distribute_or(p), distribute_or(q))
                } else if let Formula::And(And { formula1: a1, formula2: a2, .. }) = &mut *o1 {
                    let p = Formula::or((*a1).take(), *o2.clone());
                    let q = Formula::or((*a2).take(), *o2);
                    Formula::and(distribute_or(p), distribute_or(q))
                } else {
                    Formula::or(*o1, *o2)
                }
            },
            _ => formula
        }
    }

    match formula {
        Formula::And( And { formula1, formula2, .. }) => {
            Formula::and(to_cnf(*formula1), to_cnf(*formula2))
        },
        Formula::Or(Or { formula1, formula2, .. }) => {
            distribute_or(Formula::or(to_cnf(*formula1), to_cnf(*formula2))).cast()
        },
        _ => formula
    }
}


/* PNF Conversion */


/* Prenex normal form. This form guarantees all forall and exists
 * are moved to the front of a formula. Thus:
 * `∀x. (P(x) → ∃y. Q(x, y)) a`
 * becomes
 * `∀x. ∃y. (P(x) → Q(x, y))`
 * */
pub fn to_pnf(formula: Formula<Nnf>) -> Formula<Pnf> {
    fn first_non_quantified(formula: &mut Formula<Pnf>) -> &mut Formula<Pnf> {
        match formula {
            Formula::ForAll(forall) => first_non_quantified(&mut forall.formula),
            Formula::Exists(exists) => first_non_quantified(&mut exists.formula),
            _ => formula
        }
    }


    fn merge_pnfs<'a>(formula1: &'a mut Formula<Pnf>, formula2: &'a mut Formula<Pnf>)  -> &'a mut Formula<Pnf> {
        if let non_quantified @ Formula::Dummy = first_non_quantified(formula1) {
            *non_quantified = formula2.take();
            formula1
        } else {
            unreachable!("Expected Dummy at quantifier tail when merging PNFs; got nested formula")
        }
    }


    fn pnf_unop(unop: impl Fn(Formula<Pnf>) -> Formula<Pnf>, child: Formula<Nnf>) -> Formula<Pnf> {
        let mut child_pnfed = to_pnf(child);
        let non_quantified = first_non_quantified(&mut child_pnfed);
        *non_quantified = unop(non_quantified.take());
        child_pnfed
    }


    fn pnf_binop(binop: impl Fn(Formula<Pnf>, Formula<Pnf>) -> Formula<Pnf>, left: Formula<Nnf>, right: Formula<Nnf>) -> Formula<Pnf> {
        let mut left_pnfed = to_pnf(left);
        let mut right_pnfed = to_pnf(right);

        { // rename the right branch variable if there's capturing problem.
            let left_vars = first_non_quantified(&mut left_pnfed).free_vars();
            let right_vars = first_non_quantified(&mut right_pnfed).free_vars();
            let conflicts = left_vars.intersection(&right_vars).collect::<HashSet<_>>();
            let mut r = &mut right_pnfed;
            loop {
                match r {
                    Formula::Exists(_) => {
                        if let Formula::Exists(exists) = r {
                            if conflicts.contains(&exists.var) {
                                r.alpha_rename();
                            }
                        }
                        if let Formula::Exists(exists) = r {
                            r = &mut *exists.formula;
                        }
                    },
                    Formula::ForAll(_) => {
                        if let Formula::ForAll(forall) = r {
                            if conflicts.contains(&forall.var) {
                                r.alpha_rename();
                            }
                        }
                        if let Formula::ForAll(forall) = r {
                            r = &mut *forall.formula;
                        }
                    },
                    _ => break
                }
            }
        }

        let new_body = binop(
            first_non_quantified(&mut left_pnfed).take(),
            first_non_quantified(&mut right_pnfed).take());
        let quantified = merge_pnfs(&mut left_pnfed, &mut right_pnfed);
        let non_quantified = first_non_quantified(quantified);
        *non_quantified = new_body;
        quantified.take()
    }

    match formula {
        Formula::ForAll(forall) => { Formula::forall(forall.var, to_pnf(*forall.formula)) },
        Formula::Exists(exists) => { Formula::exists(exists.var, to_pnf(*exists.formula)) },
        Formula::Implies(imp) => pnf_binop(Formula::implies, *imp.formula1, *imp.formula2),
        Formula::Iff(iff) => pnf_binop(Formula::iff, *iff.formula1, *iff.formula2),
        Formula::And(and) => pnf_binop(Formula::and, *and.formula1, *and.formula2),
        Formula::Or(or) => pnf_binop(Formula::or, *or.formula1, *or.formula2),
        Formula::Not(not) => pnf_unop(Formula::not, *not.formula),
        Formula::Pred(_) => { formula.cast() },
        Formula::Dummy => { formula.cast() }
    }
}
