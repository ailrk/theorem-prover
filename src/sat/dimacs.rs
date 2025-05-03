use crate::sat::clauses::*;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

pub fn parse<R: BufRead>(reader: R) -> io::Result<Clauses> {
    let mut nclause = 0;
    let mut vec = vec![];
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if let Some('c') = line.chars().next() {
            continue;
        }
        if let Some('p') = line.chars().next() {
            let fields = line.split(' ').collect::<Vec<_>>();
            match fields[..] {
                ["p", "cnf", _, nbclause, ..] => {
                    nclause = nbclause.parse().unwrap_or(0);
                },
                _ => panic!("invalid DIMACS {:?}", line)
           };
            continue;
        }

        let mut set = HashSet::new();
        let fields = line.split(' ').collect::<Vec<_>>();
        for field in fields {
            if let Some('-') = field.chars().nth(0) {
                set.insert(Literal::neg(field[1..].to_string()));
            } else {
                set.insert(Literal::pos(field.to_string()));
            }
        }
        vec.push(set);
    }

    if nclause != 0 {
        panic!("Invalid DIMACS, incorrect number of clauses");
    }
    Ok(Clauses(vec))
}
