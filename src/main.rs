mod sat;
mod prover;
mod fol;
use fol::ast::Cnf;
use fol::ast::Grounded;
use fol::parser;
use fol::ast;


fn test_parse() {
    println!("Parser:");
    println!("{}", parser::parse("P(x)").unwrap());
    println!("{}", parser::parse("P(x, y)").unwrap());
    println!("{}", parser::parse("P(x) or not P(x)").unwrap());
    println!("{}", parser::parse("exists y . Q(y)").unwrap());
    println!("{}", parser::parse("forall x . (exists y . (P(x) => Q(y)))").unwrap());
    println!("{}", parser::parse("forall x . (not (P(x) or Q(x)))").unwrap());
    println!("{}", parser::parse("(P(x) and Q(y)) => R(z)").unwrap());
    println!("{}", parser::parse("forall x. P(x) => (Q(x) => P(x))").unwrap());
    println!("{}", parser::parse("exists x. (P(x) => forall y. P(y))").unwrap());
    println!("{}", parser::parse("exists x. (P(x) => forall y. P(y))").unwrap());
    println!("{}", parser::parse("exists x. (P(x) <=> Q(x))").unwrap());
    println!("{}", parser::parse("forall x. (P(x) <=> Q(x))").unwrap());
    println!("");
}


fn test_substitution() {
    fn substitute(input: &str, from: &str, to: &str) {
        let mut t = parser::parse(input).unwrap();
        print!("\x1b[32;1m{}\x1b[0m[{}/{}] -> ", t, from, to);
        let from_var = ast::Var::from_string(from.to_string());
        let to_var = ast::Var::from_string(to.to_string());
        t.substitute(from_var, to_var.to_term());
        println!("{}", t);
    }
    println!("Substitution:");
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
    println!("");
}


fn test_transform() {
    fn transform(input: &str) {
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
        println!("");
    }
    println!("Transform:");
    transform("P(x)");
    transform("forall a. P(f(a))");
    println!("Push negations:");
    transform("forall a. not (P(a) and P(b))");
    transform("forall a. not (P(a) or P(b))");
    transform("not (forall x. P(x))");
    transform("not (exists x. P(x))");
    transform("not (forall x. exists y. (P(x) and Q(y)))");
    println!("Eliminate arrows:");
    transform("forall a. forall b. P(a) <=> P(b)");
    transform("forall x. P(x) => Q(x)");
    println!("Skolemization:");
    transform("forall x. exists y. forall z. exists w. P(x,y,z,w)");
    transform("exists x. (forall y. (P(x, y) and Q(x)))");
    transform("forall x. exists y. exists z. (P(x) and Q(y) and R(z))");
    transform("forall x. (P(x) <=> forall y. Q(y) and exists z. R(z))");
    println!("Misc:");
    transform("forall a. P(a) and (forall b. P(b))");
    transform("forall x. (exists y. (P(x) => Q(y)))");
    transform("forall x. P(x) and forall y. Q(y)");
    transform("exists x. P(x) or exists y. Q(y)");
    transform("forall x. P(x) => exists y. Q(y)");
    transform("forall x. (P(x) => forall y. (Q(y) => exists z. R(z)))");
    transform("forall x. forall y. forall z. P(f(x)) and P(y) and P(z)");
    transform("exists x. exists y. P(x, y)");
    transform("exists x. forall y. P(x) and Q(y)");
    transform("forall x. exists y. P(x, y) and Q(y)");
    transform("exists x. forall y. exists z. P(x, y, z)");
    transform("forall x. exists y. (P(x) => Q(y))");
    transform("forall x. (exists y. (P(x) => Q(y))) and R(x)");
    transform("exists x. exists y. forall z. P(x, y, z) and Q(x, y)");
    transform("forall x. (exists y. (P(x) => exists z. Q(y, z)))");
    transform("not (exists x. exists y. P(x, y))");
    transform("forall x. (exists y. (P(x, y) => Q(y)))");
    transform("exists x. forall y. exists z. (P(x, y) and Q(z))");
    transform("forall x. (exists y. (P(x) and Q(y))) and R(x)");
    transform("forall x. exists y. (P(x) and Q(x, y))");
    println!("");
}


fn main() {
    test_parse();
    test_substitution();
    test_transform();
}
