use crate::fol::ast::*;


impl Formula<Grounded> {
    pub fn to_cnf(self) -> Formula<Cnf> { to_cnf(self) }
    pub fn to_defcnf(self) -> Formula<Cnf> { to_defcnf(self) }
}


fn to_cnf(formula: Formula<Grounded>) -> Formula<Cnf> {
    distribute_or(formula).cast()
}


fn to_defcnf(formula: Formula<Grounded>) -> Formula<Cnf> {
    todo!()
}


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
