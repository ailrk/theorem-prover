extern crate theorem_prover;
use theorem_prover::fol::parser;
use theorem_prover::fol::ast;
use theorem_prover::fol::ast::Cnf;
use theorem_prover::fol::ast::Grounded;
use theorem_prover::sat;


#[test]
fn test_parse() {
    parser::parse("P(x)").unwrap();
    parser::parse("P(x, y)").unwrap();
    parser::parse("P(x) or not P(x)").unwrap();
    parser::parse("exists y . Q(y)").unwrap();
    parser::parse("forall x . (exists y . (P(x) => Q(y)))").unwrap();
    parser::parse("forall x . (not (P(x) or Q(x)))").unwrap();
    parser::parse("(P(x) and Q(y)) => R(z)").unwrap();
    parser::parse("forall x. P(x) => (Q(x) => P(x))").unwrap();
    parser::parse("exists x. (P(x) => forall y. P(y))").unwrap();
    parser::parse("exists x. (P(x) => forall y. P(y))").unwrap();
    parser::parse("exists x. (P(x) <=> Q(x))").unwrap();
    parser::parse("forall x. (P(x) <=> Q(x))").unwrap();
}


#[test]
fn test_substitution() {
    fn substitute(input: &str, from: &str, to: &str) {
        let mut t = parser::parse(input).unwrap();
        print!("\x1b[32;1m{}\x1b[0m[{}/{}] -> ", t, from, to);
        let from_var = ast::Var::from_string(from.to_string());
        let to_var = ast::Var::from_string(to.to_string());
        t.substitute(from_var, to_var.to_term());
        println!("{}", t);
    }
    substitute("P(x)", "x", "y");
    substitute("P(x)", "x", "y");
    substitute("forall a. P(f(a))", "x", "y");
    substitute("forall x. P(x) and P(y)", "y", "x");
    substitute("exists x. P(x) and P(y)", "y", "x");
    substitute("forall a. P(a) and (forall b. P(b))", "x", "y");
    substitute("forall x. (P(x) => Q(x))", "x", "y");
    substitute("forall x. (exists y. (P(x) => Q(y)))", "x", "y");
    substitute("forall x. P(x) and forall y. Q(y)", "x", "y");
    substitute("exists x. P(x) or exists y. Q(y)", "x", "y");
    substitute("forall x. P(x) => exists y. Q(y)", "x", "y");
    substitute("not (forall x. P(x))", "x", "y");
    substitute("not (exists x. P(x))", "x", "y");
    substitute("not (forall x. exists y. (P(x) and Q(y)))", "x", "y");
    substitute("forall x. (P(x) => forall y. (Q(y) => exists z. R(z)))", "x", "y");
    substitute("forall x. forall y. forall z. P(f(x)) and P(y) and P(z)", "x", "y");
    substitute("forall x. exists y. forall z. exists w. P(x,y,z,w)", "x", "y");
}


fn pipeline(input: &str, satisfiable: bool) {
    let t = parser::parse(input).unwrap();
    println!("\x1b[32;1m{}\x1b[0m", t);
    let t = t.to_nnf();
    println!("  +-nnf---> {}", t);
    let t = t.to_pnf();
    println!("  +-pnf---> {}", t);
    let t = t.skolemize();
    println!("  +-skole-> {}", t);
    let t = t.ground();
    println!("  +-ground-> {}", t);
    let t = t.to_cnf().cast::<Grounded>();
    println!("  +-cnf-> {}", t);
    let cnfset = sat::set::CNFSet::from_formula(t.cast::<Cnf>());
    println!("  +-cnfset-> {:?}", cnfset);
    let sat = sat::dp::satisfiable_dp(cnfset);
    println!("  +-sat---> {:?}", sat);
    assert_eq!(sat, satisfiable);
    println!("");
}


#[test]
fn test_pipeline_satisfiable() {
    pipeline("P(x)", true);
    pipeline("(P(x) or not Q(x)) and (P(x) or not Q(x) or R(x)) and (not R(x))", true);
    pipeline("P(x) or Q(x)", true);
    pipeline("(P(x) or Q(x)) and (not P(x) or R(x))", true);
    pipeline("(P(x) or Q(x)) and (not Q(x) or R(x)) and (not R(x) or S(x))", true);
    pipeline("(P(x) or Q(x)) and (P(x) or not Q(x))", true);
    pipeline("(P(x)) and (Q(x)) and (R(x))", true);
    pipeline("forall a. P(f(a))", true);
    pipeline("forall a. not (P(a) and P(b))", true);
    pipeline("forall a. not (P(a) or P(b))", true);
    pipeline("not (forall x. P(x))", true);
    pipeline("not (exists x. P(x))", true);
    pipeline("not (forall x. exists y. (P(x) and Q(y)))", true);
    pipeline("forall a. forall b. P(a) <=> P(b)", true);
    pipeline("forall x. P(x) => Q(x)", true);
    pipeline("forall x. exists y. forall z. exists w. P(x,y,z,w)", true);
    pipeline("exists x. (forall y. (P(x, y) and Q(x)))", true);
    pipeline("forall x. exists y. exists z. (P(x) and Q(y) and R(z))", true);
    pipeline("forall x. (P(x) <=> forall y. Q(y) and exists z. R(z))", true);
    pipeline("forall a. P(a) and (forall b. P(b))", true);
    pipeline("forall x. (exists y. (P(x) => Q(y)))", true);
    pipeline("forall x. P(x) and forall y. Q(y)", true);
    pipeline("exists x. P(x) or exists y. Q(y)", true);
    pipeline("forall x. P(x) => exists y. Q(y)", true);
    pipeline("forall x. (P(x) => forall y. (Q(y) => exists z. R(z)))", true);
    pipeline("forall x. forall y. forall z. P(f(x)) and P(y) and P(z)", true);
    pipeline("exists x. exists y. P(x, y)", true);
    pipeline("exists x. forall y. P(x) and Q(y)", true);
    pipeline("forall x. exists y. P(x, y) and Q(y)", true);
    pipeline("exists x. forall y. exists z. P(x, y, z)", true);
    pipeline("forall x. exists y. (P(x) => Q(y))", true);
    pipeline("forall x. (exists y. (P(x) => Q(y))) and R(x)", true);
    pipeline("exists x. exists y. forall z. P(x, y, z) and Q(x, y)", true);
    pipeline("forall x. (exists y. (P(x) => exists z. Q(y, z)))", true);
    pipeline("not (exists x. exists y. P(x, y))", true);
    pipeline("forall x. (exists y. (P(x, y) => Q(y)))", true);
    pipeline("exists x. forall y. exists z. (P(x, y) and Q(z))", true);
    pipeline("forall x. (exists y. (P(x) and Q(y))) and R(x)", true);
    pipeline("forall x. (P(x) and exists y. Q(x, y)) <=> not (P(x))", true);
}


#[test]
fn test_pipeline_unsatisfiable() {
    pipeline("forall a. P(a) and not P(a)", false);
    pipeline("P(x) and not P(x)", false);
    pipeline("(P(x) or Q(x)) and (not P(x)) and (not Q(x))", false);
    pipeline("(P(x)) and (not P(x) or Q(x)) and (not Q(x))", false);
    pipeline("(P(x) or Q(x)) and (not P(x) or Q(x)) and (P(x) or not Q(x)) and (not P(x) or not Q(x))", false);
    pipeline("(P(x) or Q(x)) and (not Q(x)) and (not P(x))", false);
    pipeline("(P(x) or not P(x)) and (not P(a) or P(b)) and P(a)", false);
}
