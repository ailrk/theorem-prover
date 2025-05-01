use crate::fol::ast::*;


impl Formula<Grounded> {
    pub fn to_cnf(self) -> Formula<Cnf> { to_cnf(self) }
    pub fn to_defcnf(self) -> Formula<Cnf> { to_defcnf(self) }
}


fn to_cnf(formula: Formula<Grounded>) -> Formula<Cnf> {
    todo!()
}


fn to_defcnf(formula: Formula<Grounded>) -> Formula<Cnf> {
    todo!()
}
