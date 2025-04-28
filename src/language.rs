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
    Var(Box<Var>),
    Func(Box<Func>),
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Var {
    pub name: String,
    pub time: u32
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
    pub var: Box<Term>,
    pub formula: Box<Formula>
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Exists {
    pub var: Box<Term>,
    pub formula: Box<Formula>
}


impl Term {
    pub fn var(name: &str) -> Term {
        Term::Var(Box::new(Var { name: name.to_string(), time: 0 }))
    }

    pub fn func(name: &str, terms: Vec<Term>) -> Term {
        Term::Func(Box::new(Func { name: name.to_string(), terms }))
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        let _ = match self {
            Term::Var(var) => write!(f, "{}(Var {:?}", indent_str, var.name),
            Term::Func(func) => write!(f, "{}(Func {:?} {:?}", indent_str, func.name, func.terms)
        };
        write!(f, ")")
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
            }
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
                exists.var.fmt_with_indent(f, indent + 1)?;
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

    pub fn forall(var: Term, formula: Formula) -> Formula {
        Formula::ForAll(ForAll {
            var: Box::new(var),
            formula: Box::new(formula)
        })
    }

    pub fn exists(var: Term, formula: Formula) -> Formula {
        Formula::Exists(Exists {
            var: Box::new(var),
            formula: Box::new(formula)
        })
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
                write!(f, "(∀{}.{})", forall.var , forall.formula)
            },
            Formula::Exists(exists) => {
                write!(f, "(∃{}.{})", exists.var , exists.formula)
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
