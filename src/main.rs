mod language;
mod parser;
mod prover;


fn main() {
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
        prover::Sequent {
            left: vec![parser::parse("exists x. (P(x) => forall y. P(y))").unwrap(),
                       parser::parse("P(x, y)").unwrap()
                      ],
            right: vec![parser::parse("forall x. P(x) => (Q(x) => P(x))").unwrap(),
                        parser::parse("(P(x) and Q(y)) => R(z)").unwrap()
                       ],
            depth: 0
        });
}
