mod language;
mod parser;


fn main() {
    println!("{:?}", parser::parse("P(x)"));
    println!("{:?}", parser::parse("P(x, y)"));
    println!("{:?}", parser::parse("P(x) or not P(x)"));
    println!("{:?}", parser::parse("forall x. P(x)"));
    println!("{:?}", parser::parse("forall x. P(x) => (Q(x) implies P(x))"));
    println!("{:?}", parser::parse("exists x. (P(x) => forall y. P(y))"));
}
