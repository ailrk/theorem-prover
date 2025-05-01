use crate::fol::ast::*;


/* Prenex normal form. This form guarantees all forall and exists
 * are moved to the front of a formula. Thus:
 * `∀x. (P(x) → ∃y. Q(x, y)) a`
 * becomes
 * `∀x. ∃y. (P(x) → Q(x, y))`
 * */
pub fn to_pnf(formula: &mut Formula) {
    let taken = std::mem::take(formula);
    match taken {
        Formula::ForAll(mut forall) => { to_pnf(&mut forall.formula); *formula = Formula::ForAll(forall); },
        Formula::Exists(mut exists) => { to_pnf(&mut exists.formula); *formula = Formula::Exists(exists); },
        Formula::Implies(mut imp) => pnf_binop(formula, Formula::implies, &mut imp.formula1, &mut imp.formula2),
        Formula::Iff(mut iff) => pnf_binop(formula, Formula::iff, &mut iff.formula1, &mut iff.formula2),
        Formula::And(mut and) => pnf_binop(formula, Formula::and, &mut and.formula1, &mut and.formula2),
        Formula::Or(mut or) => pnf_binop(formula, Formula::or, &mut or.formula1, &mut or.formula2),
        Formula::Not(mut not) => pnf_unop(formula, Formula::not, &mut not.formula),
        Formula::Pred(_) => { *formula = taken },
        Formula::Dummy => {}
    }
}


fn first_non_quantified(formula: &mut Formula) -> &mut Formula {
    match formula {
        Formula::ForAll(forall) => first_non_quantified(&mut forall.formula),
        Formula::Exists(exists) => first_non_quantified(&mut exists.formula),
        _ => formula
    }
}


fn merge_pnfs<'a>(formula1: &'a mut Formula, formula2: &'a mut Formula)  -> &'a mut Formula {
    if let non_quantified @ Formula::Dummy = first_non_quantified(formula1) {
        *non_quantified = std::mem::take(formula2);
        formula1
    } else {
        unreachable!("Expected Dummy at quantifier tail when merging PNFs; got nested formula")
    }
}


fn pnf_unop(dest: &mut Formula, unop: impl Fn(Formula) -> Formula, child: &mut Formula) {
    to_pnf(child);
    let non_quantified = first_non_quantified(child);
    *non_quantified = unop(std::mem::take(non_quantified));
    *dest = std::mem::take(child);
}


fn pnf_binop(dest: &mut Formula, binop: impl Fn(Formula, Formula) -> Formula, left: &mut Formula, right: &mut Formula) {
    to_pnf(left);
    to_pnf(right);
    let new_body = binop(
        std::mem::take(first_non_quantified(left)),
        std::mem::take(first_non_quantified(right)));
    let quantified = merge_pnfs(left, right);
    let non_quantified = first_non_quantified(quantified);
    *non_quantified = new_body;
    std::mem::swap(dest, quantified);
}
