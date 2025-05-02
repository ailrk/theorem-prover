use crate::fol::ast::*;
use crate::sat::set::*;


pub fn satisfiable_dp(mut set: CNFSet) -> bool {
    loop {
        if set.0.is_empty() {
            return true;
        } else if set.0.iter().any(|clause| clause.is_empty()) {
            return false;
        } else {
            set = one_literal_rule(set);
            set = affirmative_negative_rule(set);
            set = resolution_rule(set);
        }
    }
}


fn one_literal_rule(mut set: CNFSet) -> CNFSet {
    let mut value = None;
    for clause in set.0.iter() {
        if clause.len() == 1 {
            value = clause.iter().next().cloned();
        }
    }

    if let Some(unit) = value {
        let nunit = unit.negate();
        for clause in set.0.iter_mut() {
            if clause.contains(&nunit) {
                clause.remove(&nunit);
            }
            if clause.contains(&unit) {
                clause.clear();
            }
        }

        set.0.retain(|s| !s.is_empty());
    }

    set
}


fn affirmative_negative_rule(mut set: CNFSet) -> CNFSet {
    set
}


fn resolution_rule(mut set: CNFSet) -> CNFSet {
    set
}
