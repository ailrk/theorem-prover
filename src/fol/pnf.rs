use std::collections::HashSet;

use crate::fol::ast::*;


impl Formula<Nnf> {
    pub fn to_pnf(self) -> Formula<Pnf> {
        to_pnf(self)
    }
}


/* Prenex normal form. This form guarantees all forall and exists
 * are moved to the front of a formula. Thus:
 * `∀x. (P(x) → ∃y. Q(x, y)) a`
 * becomes
 * `∀x. ∃y. (P(x) → Q(x, y))`
 * */
pub fn to_pnf(formula: Formula<Nnf>) -> Formula<Pnf> {
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


fn first_non_quantified(formula: &mut Formula<Pnf>) -> &mut Formula<Pnf> {
    match formula {
        Formula::ForAll(forall) => first_non_quantified(&mut forall.formula),
        Formula::Exists(exists) => first_non_quantified(&mut exists.formula),
        _ => formula
    }
}


fn merge_pnfs<'a>(formula1: &'a mut Formula<Pnf>, formula2: &'a mut Formula<Pnf>)  -> &'a mut Formula<Pnf> {
    if let non_quantified @ Formula::Dummy = first_non_quantified(formula1) {
        *non_quantified = std::mem::take(formula2);
        formula1
    } else {
        unreachable!("Expected Dummy at quantifier tail when merging PNFs; got nested formula")
    }
}


fn pnf_unop(unop: impl Fn(Formula<Pnf>) -> Formula<Pnf>, child: Formula<Nnf>) -> Formula<Pnf> {
    let mut child_pnfed = to_pnf(child);
    let non_quantified = first_non_quantified(&mut child_pnfed);
    *non_quantified = unop(std::mem::take(non_quantified));
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
        std::mem::take(first_non_quantified(&mut left_pnfed)),
        std::mem::take(first_non_quantified(&mut right_pnfed)));
    let quantified = merge_pnfs(&mut left_pnfed, &mut right_pnfed);
    let non_quantified = first_non_quantified(quantified);
    *non_quantified = new_body;
    std::mem::take(quantified)
}
