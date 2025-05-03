extern crate theorem_prover;
extern crate flate2;
extern crate tar;
extern crate tempfile;
use theorem_prover::sat;
use theorem_prover::sat::dimacs;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use tempfile::tempdir;
use tempfile::TempDir;


fn load_satlib(path: &str, subdir: &str, outdir: &Path) {
    let file = File::open(path).unwrap();
    let decompressor = GzDecoder::new(file);
    let mut archive = Archive::new(decompressor);
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap();
        let path_str = path.to_str().unwrap();
        if path_str.starts_with(subdir) {
            let relative_path = path.strip_prefix(subdir).unwrap();
            let outpath = outdir.join(relative_path);
            entry.unpack(&outpath).unwrap();
        }
    }
}


fn satlib_run(dir: TempDir, expect: bool) {
    for entry in fs::read_dir(dir.path()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            print!("CNF file: {}", path.display());
            let reader = BufReader::new(File::open(path).unwrap());
            let clauses = dimacs::parse(reader).expect("Failed to parse");
            let sat = sat::dp::satisfiable_dp(clauses);
            println!("  |-sat-----> {:?}, should be {:?}", sat, expect);
            assert_eq!(sat, expect);
        }
    }
}


#[test]
fn test_satlib_uf20_91() {
    let dir = tempdir().unwrap();
    load_satlib("tests/fixtures/uf20-91.tar.gz", "", dir.path());
    satlib_run(dir, true);
}


#[test]
fn test_satlib_uf50_218() {
    let dir = tempdir().unwrap();
    load_satlib("tests/fixtures/uf50-218.tar.gz", "", dir.path());
    satlib_run(dir, true);
}


#[test]
fn test_satlib_uuf50_218() {
    let dir = tempdir().unwrap();
    load_satlib("tests/fixtures/uuf50-218.tar.gz", "UUF50.218.1000/", dir.path());
    satlib_run(dir, false);
}


#[test]
fn test_satlib_uf250_1065() {
    let dir = tempdir().unwrap();
    load_satlib("tests/fixtures/uf250-1065.tar.gz", "ai/hoos/Shortcuts/UF250.1065.100/", dir.path());
    satlib_run(dir, true);
}


#[test]
fn test_satlib_uuf250_1065() {
    let dir = tempdir().unwrap();
    load_satlib("tests/fixtures/uuf250-1065.tar.gz", "UUF250.1065.100/", dir.path());
    satlib_run(dir, false);
}
