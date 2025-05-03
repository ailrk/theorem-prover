use std::collections::{HashMap, HashSet};

use crate::sat::set::*;


pub fn satisfiable_dp(mut set: CNFSet) -> bool {
    loop {
        if set.0.is_empty() {
            return true;
        } else if set.0.iter().any(|clause| clause.is_empty()) {
            return false;
        } else {
            match one_literal_rule(set) {
                Ok(s) => {
                    set = s; continue;
                },
                Err(s) => {
                    set = s;
                }
            };
            match affirmative_negative_rule(set) {
                Ok(s) => {
                    set = s; continue;
                },
                Err(s) => {
                    set = s;
                }
            }
            match resolution_rule(set) {
                Ok(s) => {
                    set = s;
                },
                Err(s) => {
                    set = s;
                }
            }
        }
    }
}


/* Remove unit clause. If we have a clause we a single literal P,
 * - Remove ¬P from other clauses.
 * - Remove clauses contains P including itself.
 * */
fn one_literal_rule(mut set: CNFSet) -> Result<CNFSet, CNFSet> {
    let mut value = None;
    for clause in set.0.iter() {
        if clause.len() == 1 {
            value = clause.iter().next().cloned();
            break
        }
    }

    if let Some(unit) = value {
        let neg = unit.negate();
        for clause in set.0.iter_mut() {
            if clause.contains(&neg) {
                clause.remove(&neg);
            }
        }
        set.0.retain(|c| !c.contains(&unit));
    } else {
        return Err(set)
    }
    Ok(set)
}


/* If a literal occurs only positively or negatively, we can remove all clauses contain them
 * while preserving satisfiability. */
fn affirmative_negative_rule(mut set: CNFSet) -> Result<CNFSet, CNFSet> {
    const POS: u8 = 0b01; const NEG: u8 = 0b10;
    let mut occurrences: HashMap<String, u8> = HashMap::new();
    for clause in set.0.iter() {
        for literal in clause.iter() {
            let k = literal.var_name().to_string();
            let mask = if literal.is_negated() { NEG } else { POS };
            match occurrences.get_mut(&k) {
                Some (occur) => { *occur |= mask },
                None => { occurrences.insert(k.to_string(), mask); }
            };
        }
    }
    let pure_occurs = occurrences
        .into_iter()
        .filter(|(_, o)| { *o == POS || *o == NEG })
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    if pure_occurs.len() == 0 { return Err(set) }
    set.0.retain(|c| {
        let mut keep = false;
        for pure in pure_occurs.iter() {
            keep = keep
                || c.contains(&Literal::pos(pure.to_string()))
                || c.contains(&Literal::neg(pure.to_string()));
        }
        !keep
    });
    Ok(set)
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
fn resolution_rule(mut set: CNFSet) -> Result<CNFSet, CNFSet> {
    let mut occurrences = HashMap::new();
    for clause in set.0.iter() {
        for literal in clause.iter() {
            if let Some(v) = occurrences.get_mut(literal.var_name()) {
                *v += 1;
            } else {
                occurrences.insert(literal.var_name().to_string(), 1);
            }
        }
    }
    let mut symbols: Vec<_> = occurrences.iter().collect();
    symbols.sort_by_key(|&(_, v)| v);

    let mut resolvents = vec![];
    for symbol in symbols.into_iter().map(|(k,_)| k) {
        let p = Literal::pos(symbol.clone());
        let n = Literal::neg(symbol.clone());
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        for (idx, clause) in set.0.iter_mut().enumerate() {
            if clause.contains(&p) {
                clause.remove(&p);
                pos.push(idx);
            } else if clause.contains(&n) {
                clause.remove(&n);
                neg.push(idx)
            }
        }
        for pidx in pos.iter_mut() {
            for nidx in neg.iter() {
                let pclause = &set.0[*pidx];
                let nclause = &set.0[*nidx];
                let resolvent = pclause.union(nclause).cloned().collect::<HashSet<_>>();
                resolvents.push(resolvent);
            }
        }
        let to_remove = pos.into_iter().chain(neg).collect::<HashSet<_>>();
        set = CNFSet(set
            .0
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| { !to_remove.contains(&idx)} )
            .map(|(_, c)| c)
            .collect::<Vec<_>>());
    }

    if resolvents.is_empty() {
        return Err (set)
    }

    set.0.append(&mut resolvents);
    Ok(set)
}
