use crate::fol::ast::*;


impl Formula<Skolemized> {
    pub fn ground(self) -> Formula<Grounded> {
        ground(self)
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
