extern crate theorem_prover;
use theorem_prover::sat;
use theorem_prover::sat::clauses::SATSolver;
use theorem_prover::sat::dimacs;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn satlib_runfile(path: &Path, expect: bool) {
    println!("CNF file: {}", path.display());
    let reader = BufReader::new(File::open(path).unwrap());
    let clauses = dimacs::parse(reader).expect("Failed to parse");
    let sat = clauses.is_satisfiable(SATSolver(sat::dp::satisfiable_dp));
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
fn test_simple_unsat() {
    let dir = Path::new("tests/fixtures/simple-unsat") ;
    satlib_run(&dir, false);
}
