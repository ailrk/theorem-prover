mod language;
mod parser;
mod transform;
mod sequent;
mod unification;
mod resolution;


fn test_parse() {
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
    println!("{}",
        sequent::Sequent {
            left: vec![parser::parse("exists x. (P(x) => forall y. P(y))").unwrap(),
                       parser::parse("P(x, y)").unwrap()
                      ],
            right: vec![parser::parse("forall x. P(x) => (Q(x) => P(x))").unwrap(),
                        parser::parse("(P(x) and Q(y)) => R(z)").unwrap()
                       ],
            depth: 0
        });
}


fn test_pnf() {
    fn pnf(input: &str) {
        let mut t = parser::parse(input).unwrap();
        print!("{} -> ", t);
        transform::pnf::to_pnf(&mut t);
        println!("{}", t);
    }

    pnf("P(x)");
    pnf("forall a. P(f(a))");
    pnf("forall a. P(a) and (forall b. P(b))");
    pnf("forall x. (P(x) => Q(x))");
    pnf("forall x. (exists y. (P(x) => Q(y)))");
    pnf("forall x. P(x) and forall y. Q(y)");
    pnf("exists x. P(x) or exists y. Q(y)");
    pnf("forall x. P(x) => exists y. Q(y)");
    pnf("not (forall x. P(x))");
    pnf("not (exists x. P(x))");
    pnf("not (forall x. exists y. (P(x) and Q(y)))");
    pnf("forall x. (P(x) => forall y. (Q(y) => exists z. R(z)))");
    pnf("forall x. forall y. forall z. P(f(x)) and P(y) and P(z)");
    pnf("forall x. exists y. forall z. exists w. P(x,y,z,w)")
}


fn main() {
    test_parse();
    test_pnf();
}
