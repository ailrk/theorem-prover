mod language;
mod parser;
mod unification;


fn main() {
    println!("{:?}", parser::lex("P or not P"));
    println!("{:?}", parser::lex("forall x. P(x) implies (Q(x) implies P(x))"));
    println!("{:?}", parser::lex("exists x. (P(x) implies forall y. P(y))"));
    println!("Hello, world!");
}
