use crate::language::ast::*;
use std::fmt::{self};


impl Term {
    pub fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        let _ = match self {
            Term::Var(var) => write!(f, "{}(Var {}", indent_str, var),
            Term::Func(func) => write!(f, "{}(Func {} {:?}", indent_str, func.name, func.terms),
            Term::Dummy => write!(f, "{}Dummy", indent_str)
        };
        write!(f, ")")
    }

    pub fn dislay(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    pub fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
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


    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
