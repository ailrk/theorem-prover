use crate::fol::ast::*;


impl Formula<Raw> {
    pub fn to_nnf(self) -> Formula<Nnf> {
        to_nnf(self)
    }
}


fn to_nnf(formula: Formula<Raw>) -> Formula<Nnf> {
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


/*
 * Eliminate Implications and Biconditionals:
 * (A → B) becomes ¬A ∨ B
 * (A ⇔  B) becomes (A ∧ B) ∨ (¬A ∧ ¬B)
 */
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


/*
 * De Morgan's Laws for connectives:
 * ¬(A ∧ B) becomes ¬A ∨ ¬B
 * ¬(A ∨ B) becomes ¬A ∧ ¬B
 *
 * Quantifier negation (handle negations of quantifiers):
 * ¬∀x.M becomes ∃x.¬M
 * ¬∃x.M becomes ∀x.¬M
 */
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
