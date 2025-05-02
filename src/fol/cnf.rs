use crate::fol::ast::*;


impl Formula<Grounded> {
    pub fn to_cnf(self) -> Formula<Cnf> { to_cnf(self).cast() }
    pub fn to_defcnf(self) -> Formula<Cnf> { to_defcnf(self).cast() }
}


/*  p ∧ q => cnf(p) ∧ cnf(q)
 *  p ∨ q => distribute_or(cnf(p) ∨ cnf(q))
 *  Because it's already grounded this covers all cases.
 */
fn to_cnf(formula: Formula<Grounded>) -> Formula<Grounded> {
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


fn to_defcnf(formula: Formula<Grounded>) -> Formula<Grounded> {
    todo!()
}


/* p ∨ (q ∧ r) ⇔  (p ∨ q) ∧  (p ∨ r)
 * (p ∧ q) ∨ r ⇔  (p ∨ r) ∧ (q ∨ r).
 */
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
