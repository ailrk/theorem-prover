use std::collections::{HashMap, HashSet};

use crate::sat::clauses::*;


/* The original davis putnam procedure. This only terminates on unsat cnf.
 * */
pub fn satisfiable_dp(clauses: Clauses) -> bool {
    let mut clauses = clauses;
    loop {
        if clauses.0.is_empty() {
            return true;
        }
        if clauses.0.iter().any(|clause| clause.is_empty()) {
            return false;
        }

        match unit_propagation_rule(clauses) {
            Ok(s) => {
                clauses = s; continue;
            },
            Err(s) => {
                clauses = s;
            }
        };

        match affirmative_negative_rule(clauses) {
            Ok(s) => {
                clauses = s; continue;
            },
            Err(s) => {
                clauses = s;
            }
        }

        match resolution_rule(clauses) {
            Ok(s) => {
                clauses = s; continue;
            },
            Err(s) => {
                clauses = s;
            }
        }
    }
}


/* Remove unit clause. If we have a clause with a single literal P,
 * - Remove ¬P from other clauses.
 * - Remove clauses contains P including itself.
 * */
pub fn unit_propagation_rule(mut clauses: Clauses) -> Result<Clauses, Clauses> {
    let mut value = None;
    for clause in clauses.0.iter() {
        if clause.len() == 1 {
            value = clause.iter().next().cloned();
            break
        }
    }

    if let Some(unit) = value {
        let neg = unit.negate();
        let remove = |c: &mut Clause| if c.contains(&neg) { c.remove(&neg); } else {};
        clauses.0.iter_mut().for_each(remove);
        clauses.0.retain(|c| !c.contains(&unit));
    } else {
        return Err(clauses)
    }
    Ok(clauses)
}


/* If a literal occurs only positively or negatively, we can remove all clauses contain them
 * while preserving satisfiability. */
pub fn affirmative_negative_rule(mut clauses: Clauses) -> Result<Clauses, Clauses> {
    const POS: u8 = 0b01; const NEG: u8 = 0b10;
    let mut occurrences: HashMap<String, u8> = HashMap::new();
    for clause in clauses.iter() {
        for literal in clause.iter() {
            let k = literal.var_name().to_string();
            let mask = if literal.is_negated() { NEG } else { POS };
            match occurrences.get_mut(&k) {
                Some (occur) => { *occur |= mask },
                None => { occurrences.insert(k.to_string(), mask); }
            };
        }
    }
    let to_remove = occurrences
        .into_iter()
        .filter(|(_, o)| { *o == POS || *o == NEG })
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    if to_remove.len() == 0 { return Err(clauses) }
    clauses.retain(|c| {
        let mut keep = true;
        for pure in to_remove.iter() {
            if c.contains(&Literal::pos(pure.to_string())) || c.contains(&Literal::neg(pure.to_string())) {
                keep = false;
            }
        }
        keep
    });
    Ok(clauses)
}


/* If a literal p exists positively in one clause and negatively in another, we can resolve
 * these two clauses to remove p.
 *
 * e.g
 * With the following cnfs
 * `C1:P ∨ Q, C2:¬P ∨ R, C3:¬P ∨ S, C4:P ∨ T`
 *
 * If we try to resolve on P, we first split clauses into positive P and negative P (ignore
 * ones doesn't containt P at all), then we get
 * `(C1:P ∨ Q, C4:P ∨ T)` and `(C2:¬P ∨ R, C3:¬P ∨ S)`
 *
 * We first removes clauses above from the original CNFs, then we try to resolve all
 * pairs get new resolvents:
 * `C1xC2: Q ∨ R, C1XC3: Q ∨ S, C4xC2: T ∨ R, C4xC2: T ∨ S`
 *
 * These resolvents are then being added back into the CNF for the next iteration.
 *
 * DP will simply try to iterate through all symbols in the cnf and try to resolve them in
 * turn. There are better way to pick literal in more optimized algorithms.
 * */
pub fn resolution_rule(mut clauses: Clauses) -> Result<Clauses, Clauses> {
    let mut occurrences = HashMap::new();
    for clause in clauses.iter() {
        for literal in clause.iter() {
            if let Some(v) = occurrences.get_mut(literal.var_name()) {
                *v += 1;
            } else {
                occurrences.insert(literal.var_name().to_string(), 1);
            }
        }
    }
    let mut symbols: Vec<_> = occurrences.iter().collect();
    symbols.sort_by_key(|&(_, v)| -v);

    let mut resolvents = Clauses::new();
    let mut need_to_remove = HashSet::new();


    for symbol in symbols.into_iter().map(|(k,_)| k) {
        let p = Literal::pos(symbol.clone());
        let n = Literal::neg(symbol.clone());

        let mut pos = Vec::new();
        let mut neg = Vec::new();

        clauses.iter_mut().enumerate().for_each(|(idx, clause)| {
            if clause.contains(&p) {
                pos.push(idx);
            } else if clause.contains(&n) {
                neg.push(idx)
            }
        });
        for pidx in pos.iter_mut() { // cross over
            for nidx in neg.iter() {
                let pclause = &clauses[*pidx];
                let nclause = &clauses[*nidx];
                let mut resolvent = pclause.union(nclause).cloned().collect::<Clause>();
                resolvent.remove_trivals();
                resolvents.push(resolvent);
            }
        }

        need_to_remove.extend(pos.into_iter().chain(neg).collect::<HashSet<_>>());
    }

    clauses = clauses.0
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| { !need_to_remove.contains(&idx)} )
        .map(|(_, c)| c)
        .collect();
    clauses = remove_trivial_clauses(clauses);
    clauses.append(&mut resolvents);
    Ok(clauses)
}


// Remove trivial tautology from clauses. If removing the tautology yeilds an empty clause, we
// remove the empty clause completely.
// This function preserves empty clauses that are already there.
fn remove_trivial_clauses(mut clauses: Clauses) -> Clauses {
    let n_empty_clauses = clauses.iter().filter(|c| c.len() == 0).count();
    clauses.iter_mut().for_each(|c| c.remove_trivals());
    clauses.retain(|clause| !clause.is_empty());
    clauses.append(&mut std::iter::repeat_n(Clause::new(), n_empty_clauses).collect::<Clauses>());
    clauses
}
