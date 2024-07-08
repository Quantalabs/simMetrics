use openbabel::fingerprint;
use openbabel::fingerprint::Kind;
use openbabel::molecule::Molecule;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_from_file(f: &str) -> Vec<Molecule> {
    let file = BufReader::new(File::open(f).unwrap());

    file.lines()
        .map(|x| Molecule::new_from_smiles(&x.unwrap()))
        .collect::<Vec<_>>()
}

pub fn gen_fps(fp: Kind, f: &str) -> Vec<Vec<u32>> {
    let fpg = fingerprint::FingerprintGenerator::new(fp);
    let mols = load_from_file(f);

    mols.iter().map(|x| fpg.get_fingerprint(x)).collect()
}

pub fn load_plain(f: &str) -> Vec<String> {
    BufReader::new(File::open(f).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
}
