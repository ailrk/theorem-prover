extern crate theorem_prover;
use theorem_prover::fol::ast::Formula;
use theorem_prover::fol::parser;
use theorem_prover::fol::ast;
use theorem_prover::fol::ast::Cnf;
use theorem_prover::fol::ast::Grounded;
use theorem_prover::sat;
use theorem_prover::sat::clauses::SATSolver;


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
    let sat = clauses.is_satisfiable(SATSolver(sat::dp::satisfiable_dp));
    println!("  +-sat-----> {:?}, should be {:?}", sat, is_satisfiable);
    assert_eq!(sat, is_satisfiable);
    println!("");
}


fn tautology(input: &str, is_valid: bool) {
    let t = to_cnf(input);
    let clauses = sat::clauses::Clauses::from_formula(t.cast::<Cnf>());
    println!("  +-clauses-> {:?}", clauses);
    let valid = clauses.is_valid(SATSolver(sat::dp::satisfiable_dp));
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
fn test_satisfiable_unsatisfiable() {
    satisfiable("forall a. P(a) and not P(a)", false);
    satisfiable("P(x) and not P(x)", false);
    satisfiable("(P(x) or Q(x)) and (not P(x)) and (not Q(x))", false);
    satisfiable("(P(x)) and (not P(x) or Q(x)) and (not Q(x))", false);
    satisfiable("(P(x) or Q(x)) and (not P(x) or Q(x)) and (P(x) or not Q(x)) and (not P(x) or not Q(x))", false);
    satisfiable("(P(x) or Q(x)) and (not Q(x)) and (not P(x))", false);
    satisfiable("(P(x) or not P(x)) and (not P(a) or P(b)) and P(a)", false);
}
