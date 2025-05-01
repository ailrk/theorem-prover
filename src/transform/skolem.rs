use crate::fol::ast::*;


pub struct SkolemState {
    counter: usize,
}

impl SkolemState {
    pub fn new() -> Self {
        SkolemState { counter: 0 }
    }

    pub fn fresh_skolem(&mut self) -> String {
        let name = format!("sk{}", self.counter);
        self.counter += 1;
        name
    }
}


/* Perform skolemization to eliminate existial quantifiers.
 * The input `formula` needs to be in PNF form.
 * Skolemized formula will have no existential quantifiers, the existing one will be
 * replaced by a fresh skolem function that depends on all universal quantifiers.
 * e.g.
 * `forall a. forall b. exists c P(a) and P(b) and P(c)`
 * becomes
 * `forall a. forall b. P(a) and P(b) and P(sk1(a, b))`
 *
 * The resulting formula is Equisatisfiable to the original one.
 * */
pub fn skolemize(formula: &mut Formula, drop_univeral: bool) {
    let mut uvars = vec![];
    collect_universal_vars(&formula, &mut uvars);
    let uvars = uvars;
    let mut state = SkolemState::new();
    loop {
        let taken = std::mem::take(formula);
        match taken {
            Formula::Exists(mut exists) => {
                let var = exists.var.clone();
                let mut skolem = fresh_skolem(&uvars, &mut state);
                first_non_quantified(&mut exists.formula).substitute(var, &mut skolem);
                *formula = *exists.formula;
            },
            Formula::ForAll(forall) => {
                *formula = *forall.formula;
            },
            _ => {
                *formula = taken;
                break;
            }
        }
    }

    if !drop_univeral { // rewrap
        for var in uvars.into_iter().rev() {
            let rest = std::mem::take(formula);
            *formula = Formula::forall(Var::from_string(var), rest);
        }
    }
}


fn first_existential(formula: &mut Formula) -> &mut Formula {
    match formula {
        Formula::ForAll(forall) => first_existential(&mut forall.formula),
        Formula::Exists(_) => formula,
        _ => unreachable!("Not in PNF form")
    }
}


fn first_non_quantified(formula: &mut Formula) -> &mut Formula {
    match formula {
        Formula::ForAll(forall) => first_non_quantified(&mut forall.formula),
        Formula::Exists(exists) => first_non_quantified(&mut exists.formula),
        _ => formula
    }
}


fn fresh_skolem(uvars: &[String], state: &mut SkolemState) -> Term {
    if uvars.len() == 0 {
        let name = state.fresh_skolem();
        Term::var(&name)

    } else {
        let fname = state.fresh_skolem();
        let params = uvars.iter().map(|str| Term::var(str)).collect::<Vec<Term>>();
        Term::func(&fname, params)
    }
}


fn collect_universal_vars(formula: &Formula, out: &mut Vec<String>) {
    debug_assert!(matches!(formula, Formula::ForAll(_) | Formula::Exists(_) | _), "Expected PNF formula");
    match formula {
        Formula::ForAll(forall) => {
            out.push(forall.var.name.clone());
            collect_universal_vars(&forall.formula, out);
        },
        _ => {}
    };
}
