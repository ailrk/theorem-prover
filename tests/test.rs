extern crate theorem_prover;
use theorem_prover::fol::ast::Formula;
use theorem_prover::fol::ast::Raw;
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


fn to_cnf(input: &str) -> Formula<Cnf> {
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
    println!("  +-cnf----> {}", t);
    t.cast::<Cnf>()
}


fn satisfiable(input: &str, is_satisfiable: bool) {
    let t = to_cnf(input);
    let clauses = sat::clauses::Clauses::from_formula(t.cast::<Cnf>());
    println!("  +-clauses-> {:?}", clauses);
    let sat = sat::dp::satisfiable_dp(clauses);
    println!("  +-sat-----> {:?}, should be {:?}", sat, is_satisfiable);
    assert_eq!(sat, is_satisfiable);
    println!("");
}


fn tautology(input: &str, is_valid: bool) {
    let t = Formula::not(to_cnf(input)).cast::<Raw>().to_nnf().to_pnf().skolemize().ground().to_cnf();
    let clauses = sat::clauses::Clauses::from_formula(t.cast::<Cnf>());
    println!("  +-clauses-> {:?}", clauses);
    let valid = !sat::dp::satisfiable_dp(clauses);
    println!("  +-taut----> {:?}, should be {:?}", valid, is_valid);
    assert_eq!(valid, is_valid);
    println!("");
}


#[test]
fn test_tautologies() {
    tautology("P(x) or not P(x)", true);  // classical tautology
    tautology("P(x) => P(x)", true);
    tautology("forall x. P(x) => P(x)", true);
    tautology("P(x) and not P(x)", false);  // contradiction
    tautology("P(x)", false);              // satisfiable but not tautology
}


#[test]
fn test_satisfiable_satisfiable() {
    satisfiable("P(x)", true);
    satisfiable("(P(x) or not Q(x)) and (P(x) or not Q(x) or R(x)) and (not R(x))", true);
    satisfiable("P(x) or Q(x)", true);
    satisfiable("(P(x) or Q(x)) and (not P(x) or R(x))", true);
    satisfiable("(P(x) or Q(x)) and (not Q(x) or R(x)) and (not R(x) or S(x))", true);
    satisfiable("(P(x) or Q(x)) and (P(x) or not Q(x))", true);
    satisfiable("(P(x)) and (Q(x)) and (R(x))", true);
    satisfiable("forall a. P(f(a))", true);
    satisfiable("forall a. not (P(a) and P(b))", true);
    satisfiable("forall a. not (P(a) or P(b))", true);
    satisfiable("not (forall x. P(x))", true);
    satisfiable("not (exists x. P(x))", true);
    satisfiable("not (forall x. exists y. (P(x) and Q(y)))", true);
    satisfiable("forall a. forall b. P(a) <=> P(b)", true);
    satisfiable("forall x. P(x) => Q(x)", true);
    satisfiable("forall x. exists y. forall z. exists w. P(x,y,z,w)", true);
    satisfiable("exists x. (forall y. (P(x, y) and Q(x)))", true);
    satisfiable("forall x. exists y. exists z. (P(x) and Q(y) and R(z))", true);
    satisfiable("forall x. (P(x) <=> forall y. Q(y) and exists z. R(z))", true);
    satisfiable("forall a. P(a) and (forall b. P(b))", true);
    satisfiable("forall x. (exists y. (P(x) => Q(y)))", true);
    satisfiable("forall x. P(x) and forall y. Q(y)", true);
    satisfiable("exists x. P(x) or exists y. Q(y)", true);
    satisfiable("forall x. P(x) => exists y. Q(y)", true);
    satisfiable("forall x. (P(x) => forall y. (Q(y) => exists z. R(z)))", true);
    satisfiable("forall x. forall y. forall z. P(f(x)) and P(y) and P(z)", true);
    satisfiable("exists x. exists y. P(x, y)", true);
    satisfiable("exists x. forall y. P(x) and Q(y)", true);
    satisfiable("forall x. exists y. P(x, y) and Q(y)", true);
    satisfiable("exists x. forall y. exists z. P(x, y, z)", true);
    satisfiable("forall x. exists y. (P(x) => Q(y))", true);
    satisfiable("forall x. (exists y. (P(x) => Q(y))) and R(x)", true);
    satisfiable("exists x. exists y. forall z. P(x, y, z) and Q(x, y)", true);
    satisfiable("forall x. (exists y. (P(x) => exists z. Q(y, z)))", true);
    satisfiable("not (exists x. exists y. P(x, y))", true);
    satisfiable("forall x. (exists y. (P(x, y) => Q(y)))", true);
    satisfiable("exists x. forall y. exists z. (P(x, y) and Q(z))", true);
    satisfiable("forall x. (exists y. (P(x) and Q(y))) and R(x)", true);
    satisfiable("forall x. (P(x) and exists y. Q(x, y)) <=> not (P(x))", true);
}


#[test]
fn test_satisfiable_unsatisfiable() {
    satisfiable("forall a. P(a) and not P(a)", false);
    satisfiable("P(x) and not P(x)", false);
    satisfiable("(P(x) or Q(x)) and (not P(x)) and (not Q(x))", false);
    satisfiable("(P(x)) and (not P(x) or Q(x)) and (not Q(x))", false);
    satisfiable("(P(x) or Q(x)) and (not P(x) or Q(x)) and (P(x) or not Q(x)) and (not P(x) or not Q(x))", false);
    satisfiable("(P(x) or Q(x)) and (not Q(x)) and (not P(x))", false);
    satisfiable("(P(x) or not P(x)) and (not P(a) or P(b)) and P(a)", false);
}
