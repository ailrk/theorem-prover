use std::collections::HashSet;
use std::fmt::{self};
use std::hash::Hash;


#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Formula {
    Pred(Pred),
    Not(Not),
    And(And),
    Or(Or),
    Implies(Implies),
    ForAll(ForAll),
    Exists(Exists),
    Dummy,
}


#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Term {
    Var(Var),
    Func(Func),
    Dummy,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Var {
    pub name: String,
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Func {
    pub name: String,
    pub terms: Vec<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Pred {
    pub name: String,
    pub terms: Vec<Term>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Not {
    pub formula: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct And {
    pub formula1: Box<Formula>,
    pub formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Or {
    pub formula1: Box<Formula>,
    pub formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Implies {
    pub formula1: Box<Formula>,
    pub formula2: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ForAll {
    pub var: Var,
    pub formula: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Exists {
    pub var: Var,
    pub formula: Box<Formula>
}


impl Var {
    pub fn from_string(name: String) -> Self { Var{ name }}
    pub fn to_term(&self) -> Term { Term::Var(self.clone()) }
}


impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}


impl Term {
    pub fn var(name: &str) -> Term {
        Term::Var(Var { name: name.to_string() })
    }

    pub fn func(name: &str, terms: Vec<Term>) -> Term {
        Term::Func(Func { name: name.to_string(), terms })
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        let _ = match self {
            Term::Var(var) => write!(f, "{}(Var {}", indent_str, var),
            Term::Func(func) => write!(f, "{}(Func {} {:?}", indent_str, func.name, func.terms),
            Term::Dummy => write!(f, "{}Dummy", indent_str)
        };
        write!(f, ")")
    }

    pub fn free_vars(&self) -> HashSet<Var> {
        let mut vars = HashSet::new();
        self.collect_free_vars(&mut vars);
        vars
    }

    fn collect_free_vars(&self, vars: &mut HashSet<Var>) {
        match self {
            Term::Var(var) => {
                vars.insert(var.clone());
            },
            Term::Func(func) => {
                for term in &func.terms {
                    term.collect_free_vars(vars);
                }
            },
            Term::Dummy => {}
        }
    }

    pub fn substitute(&mut self, from: Var, to: &mut Term) {
        let taken = std::mem::take(self);
        match taken {
            Term::Var(ref var) => {
                if *var == from {
                    *self = std::mem::take(to)
                } else {
                    *self = taken
                }
            },
            Term::Func(mut func) => {
                for term in func.terms.iter_mut() {
                    term.substitute(from.clone(), to)
                }
                *self = Term::func( &func.name, func.terms)
            }
            Term::Dummy => {}
        }
    }
}


impl Default for Term {
    fn default() -> Self {
        Term::Dummy
    }
}


impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0) // Start with indentation level 0
    }
}


impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "{}", var.name),
            Term::Func(func) => {
                write!(f, "{}(", func.name)?;
                for (idx, param) in func.terms.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if idx < func.terms.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, ")")
            },
            Term::Dummy => write!(f, "Dummy"),
        }
    }
}


impl Formula {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        match self {
            Formula::Pred(pred) => {
                write!(f, "{}(Pred {:?} {:?}", indent_str, pred.name, pred.terms)?;
            },
            Formula::Not(not) => {
                write!(f, "{}(Not {:?}", indent_str, not.formula)?;
            },
            Formula::And(and) => {
                write!(f, "{}(And\n", indent_str)?;
                and.formula1.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n")?;
                and.formula2.fmt_with_indent(f, indent + 1)?;
            },
            Formula::Or(or) => {
                write!(f, "{}(Or\n", indent_str)?;
                or.formula1.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n")?;
                or.formula2.fmt_with_indent(f, indent + 1)?;
            },
            Formula::Implies(imp) => {
                write!(f, "{}(=>\n", indent_str)?;
                imp.formula1.fmt_with_indent(f, indent + 1)?;
                write!(f, "\n")?;
                imp.formula2.fmt_with_indent(f, indent + 1)?;
            },
            Formula::ForAll(forall) => {
                write!(f, "{}(Forall {:?}\n", indent_str, forall.var)?;
                forall.formula.fmt_with_indent(f, indent + 1)?;
            },
            Formula::Exists(exists) => {
                write!(f, "{}(Exists\n", indent_str)?;
                exists.var.to_term().fmt_with_indent(f, indent + 1)?;
                write!(f, "\n")?;
                exists.formula.fmt_with_indent(f, indent + 1)?;
            }
            Formula::Dummy => {
                write!(f, "{}Dummy", indent_str)?;
            }
        };
        write!(f, ")")
    }

    pub fn pred(name: &str, terms: Vec<Term>) -> Formula {
        Formula::Pred(Pred {
            name: name.to_string(),
            terms
        })
    }

    pub fn implies(formula1: Formula, formula2: Formula) -> Formula {
        Formula::Implies(Implies {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        })
    }

    pub fn or(formula1: Formula, formula2: Formula) -> Formula {
        Formula::Or(Or {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        })
    }

    pub fn and(formula1: Formula, formula2: Formula) -> Formula {
        Formula::And(And {
            formula1: Box::new(formula1),
            formula2: Box::new(formula2)
        })
    }

    pub fn not(formula: Formula) -> Formula {
        Formula::Not(Not {
            formula: Box::new(formula)
        })
    }

    pub fn forall(var: Var, formula: Formula) -> Formula {
        Formula::ForAll(ForAll {
            var,
            formula: Box::new(formula)
        })
    }

    pub fn exists(var: Var, formula: Formula) -> Formula {
        Formula::Exists(Exists {
            var,
            formula: Box::new(formula)
        })
    }

    pub fn free_vars(&self) -> HashSet<Var> {
        let mut vars = HashSet::new();
        self.collect_free_vars(&mut vars);
        vars
    }

    fn collect_free_vars(&self, vars: &mut HashSet<Var>) {
        match self {
            Formula::Pred(pred) => {
                for term in &pred.terms {
                    vars.extend(term.free_vars())
                }
            },
            Formula::Not(not) => {
                vars.extend(not.formula.free_vars())
            },
            Formula::And(and) => {
                vars.extend(and.formula1.free_vars());
                vars.extend(and.formula2.free_vars());
            },
            Formula::Or(or) => {
                vars.extend(or.formula1.free_vars());
                vars.extend(or.formula2.free_vars());
            },
            Formula::Implies(imp) => {
                vars.extend(imp.formula1.free_vars());
                vars.extend(imp.formula2.free_vars());
            },
            Formula::ForAll(forall) => {
                let mut inner = forall.formula.free_vars();
                inner.remove(&forall.var);
                vars.extend(inner)
            },
            Formula::Exists(exists) => {
                let mut inner = exists.formula.free_vars();
                inner.remove(&exists.var);
                vars.extend(inner)
            },
            Formula::Dummy => {}
        }
    }

    /* Replace var with term. */
    pub fn substitute(&mut self, from: Var, to: &mut Term) {
        match self {
            Formula::Pred(pred) => {
                for term in pred.terms.iter_mut() {
                    term.substitute(from.clone(), to);
                }
            },
            Formula::Not(not) => not.formula.substitute(from.clone(), to),
            Formula::And(and) => {
                and.formula1.substitute(from.clone(), to);
                and.formula2.substitute(from.clone(), to);
            }
            Formula::Or(or) => {
                or.formula1.substitute(from.clone(), to);
                or.formula2.substitute(from.clone(), to);
            },
            Formula::Implies(imp) => {
                imp.formula1.substitute(from.clone(), to);
                imp.formula2.substitute(from.clone(), to);
            },
            /* We need to perform alpha-renaming on quantifier cases. e.g For a given substitution [from/to]
             * on formula `forall x. M`, if to.free_vars() contains x, we need to rename x to avoid capture.
             * So `forall x. M` -> `forall x1. M[x/x1]`.
             * Then we can perform the original substitution safely.
             */
            Formula::ForAll(_) => {
                let mut taken = std::mem::take(self);
                if let Formula::ForAll(ref mut forall) = taken {
                    if forall.var == from { // bounded
                    } else if to.free_vars().contains(&forall.var){
                        let free_vars = self.free_vars();
                        let fresh = fresh_name(&from, &free_vars);
                        forall.var = fresh;
                        forall.formula.substitute(from.clone(), to);
                    } else {
                        forall.formula.substitute(from.clone(), to);
                    }
                }
                *self = std::mem::take(&mut taken)
            },
            Formula::Exists(_) => {
                let mut taken = std::mem::take(self);
                if let Formula::Exists(ref mut exists) = taken {
                    if exists.var == from { // bounded
                    } else if to.free_vars().contains(&exists.var){
                        let free_vars = self.free_vars();
                        let fresh = fresh_name(&from, &free_vars);
                        exists.var = fresh;
                        exists.formula.substitute(from.clone(), to);
                    } else {
                        exists.formula.substitute(from.clone(), to);
                    }
                }
                *self = std::mem::take(&mut taken)
            },
            Formula::Dummy => {},
        }
    }
}


impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0) // Start with indentation level 0
    }
}


impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Pred(pred) => {
                write!(f, "{}(", pred.name)?;
                for (idx, param) in pred.terms.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if idx < pred.terms.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, ")")
            },
            Formula::Not(not) => {
                write!(f, "¬{}", not.formula)
            },
            Formula::And(and) => {
                write!(f, "({} ∧ {})", and.formula1, and.formula2)
            },
            Formula::Or(or) => {
                write!(f, "({} ∨ {})", or.formula1, or.formula2)
            },
            Formula::Implies(imp) => {
                write!(f, "({} → {})", imp.formula1, imp.formula2)
            },
            Formula::ForAll(forall) => {
                write!(f, "(∀{}.{})", forall.var.to_term(), forall.formula)
            },
            Formula::Exists(exists) => {
                write!(f, "(∃{}.{})", exists.var.to_term(), exists.formula)
            },
            Formula::Dummy => {
                write!(f, "Dummy")
            }
        }
    }
}


impl Default for Formula {
    fn default() -> Self {
        Formula::Dummy
    }
}


fn into_unions(sets: Vec<HashSet<Term>>) -> HashSet<Term> {
    sets.into_iter().fold(HashSet::new(), |b, a| -> HashSet<Term> {
        a.into_iter().chain(b.into_iter()).collect()
    })
}


fn fresh_name(base: &Var, taken: &HashSet<Var>) -> Var {
    for i in 0.. {
        let name = Var::from_string(format!("{}_{}", base.to_term(), i));
        if !taken.contains(&name) {
            return name;
        }
    }
    unreachable!()
}
