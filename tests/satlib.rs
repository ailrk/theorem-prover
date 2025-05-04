extern crate theorem_prover;
use theorem_prover::sat;
use theorem_prover::sat::dimacs;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn satlib_runfile(path: &Path, expect: bool) {
    println!("CNF file: {}", path.display());
    let reader = BufReader::new(File::open(path).unwrap());
    let clauses = dimacs::parse(reader).expect("Failed to parse");
    let sat = sat::dp::satisfiable_dp(clauses);
    println!("  |-sat-----> {:?}, should be {:?}", sat, expect);
    assert_eq!(sat, expect);
}


fn satlib_run(dir: &Path, expect: bool) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            satlib_runfile(&path, expect);
        }
    }
}


#[test]
fn test_satlib1() {
    let path = Path::new("/home/fatmonad/test/sats/uf20-91/uf20-0821.cnf");
    satlib_runfile(&path, true);
}


#[test]
fn test_resolution() {
    let path = Path::new("/home/fatmonad/test/sats/resolution1.cnf");
    let reader = BufReader::new(File::open(path).unwrap());
    let clauses = dimacs::parse(reader).expect("Failed to parse");
    println!("1:: {}", clauses);
    if let Ok(clauses) = sat::dp::resolution_rule(clauses) {
        println!("2:: {}", clauses);
        if let Ok(clauses) = sat::dp::resolution_rule(clauses) {
            println!("3:: {}", clauses);
            if let Ok(clauses) = sat::dp::resolution_rule(clauses) {
                println!("4:: {}", clauses);
            }
        }
    }

    // satlib_runfile(&path, true);
}


#[test]
fn test_simple_sat() {
    let dir = Path::new("tests/fixtures/simple-sat") ;
    satlib_run(&dir, true);
}


#[test]
fn test_simple_unsat() {
    let dir = Path::new("tests/fixtures/simple-unsat") ;
    satlib_run(&dir, false);
}
