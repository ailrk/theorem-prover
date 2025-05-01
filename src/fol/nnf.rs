use crate::fol::ast::*;


impl Formula<Raw> {
    pub fn to_nnf(self) -> Formula<Nnf> {
        to_nnf(self)
    }
}


fn to_nnf(formula: Formula<Raw>) -> Formula<Nnf> {
    push_negations(eliminate_arrows(formula)).cast()
}


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


fn push_negations(formula: Formula<Raw>) -> Formula<Raw> {
    match formula {
        Formula::Not(Not { formula, .. }) => {
            match *formula {
                Formula::And(And { formula1, formula2, .. }) => Formula::and(Formula::not(push_negations(*formula1)), Formula::not(push_negations(*formula2))),
                Formula::Or(Or { formula1, formula2, .. }) => Formula::or(Formula::not(push_negations(*formula1)), Formula::not(push_negations(*formula2))),
                Formula::ForAll(ForAll { var, formula, .. }) => Formula::exists(var, Formula::not(*formula)),
                Formula::Exists(Exists { var, formula, .. }) => Formula::forall(var, Formula::not(*formula))
                ,
                _ => Formula::not(push_negations(*formula))
            }
        },
        Formula::And(And { formula1, formula2, .. }) => Formula::and(push_negations(*formula1), push_negations(*formula2)),
        Formula::Or(Or { formula1, formula2, .. }) => Formula::or(push_negations(*formula1), push_negations(*formula2)),
        Formula::ForAll(ForAll { var, formula, .. } ) => Formula::forall(var, push_negations(*formula)),
        Formula::Exists(Exists { var, formula, .. }) => Formula::exists(var, push_negations(*formula)),
        Formula::Implies(Implies { formula1, formula2, .. }) => Formula::implies(push_negations(*formula1), push_negations(*formula2)),
        Formula::Iff(Iff { formula1, formula2, .. }) => Formula::iff(push_negations(*formula1), push_negations(*formula2)),
        Formula::Pred(_) => formula,
        Formula::Dummy => formula,
    }
}
