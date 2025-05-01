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


impl Formula<Pnf> {
    pub fn skolemize(self) -> Formula<Skolemized> {
        skolemize(self)
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
fn skolemize(mut formula: Formula<Pnf> ) -> Formula<Skolemized> {
    let mut uvars = vec![];
    collect_universal_vars(&formula, &mut uvars);
    let uvars = uvars;
    let mut state = SkolemState::new();
    loop {
        match formula {
            Formula::Exists(mut exists) => {
                let var = exists.var.clone();
                let skolem = fresh_skolem(&uvars, &mut state);
                first_non_quantified(&mut exists.formula).substitute(var, skolem);
                formula = *exists.formula;
            },
            Formula::ForAll(forall) => {
                formula = *forall.formula;
            },
            _ => {
                break;
            }
        }
    }

    for var in uvars.into_iter().rev() {
        let rest = formula;
        formula = Formula::forall(Var::from_string(var), rest);
    }
    formula.cast()
}


fn first_non_quantified(formula: &mut Formula<Pnf>) -> &mut Formula<Pnf> {
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


fn collect_universal_vars(formula: &Formula<Pnf>, out: &mut Vec<String>) {
    match formula {
        Formula::ForAll(forall) => {
            out.push(forall.var.name.clone());
            collect_universal_vars(&forall.formula, out);
        },
        Formula::Exists(exists) => {
            collect_universal_vars(&exists.formula, out);
        }
        _ => {}
    };
}
