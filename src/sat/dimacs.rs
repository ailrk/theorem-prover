use crate::sat::clauses::*;
use std::io;
use std::io::BufRead;

pub fn parse<R: BufRead>(reader: R) -> io::Result<Clauses> {
    let mut clauses = Clauses::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if let Some('c') = line.trim().chars().next() {
            continue;
        }
        if let Some('p') = line.trim().chars().next() {
            let fields = line.trim().split(' ').collect::<Vec<_>>();
            match fields[..] {
                ["p", "cnf", _, _, ..] => {},
                _ => panic!("invalid DIMACS {:?}", line)
           };
            continue;
        }
        if let Some('%') = line.trim().chars().next() {
            break;
        }

        let mut set = Clause::new();
        let fields = line.trim().split(' ').collect::<Vec<_>>();
        for field in fields.iter() {
            if let "0" = *field {
                break;
            }
            if let Some('-') = field.chars().nth(0) {
                set.insert(Literal::neg(field[1..].to_string()));
            } else {
                set.insert(Literal::pos(field.to_string()));
            }
        }
        clauses.push(set);
    }
    Ok(clauses)
}
