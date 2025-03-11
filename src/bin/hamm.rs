use std::env;
use std::fs;

fn hamm(seq1: &str, seq2: &str) -> usize {
    seq1.chars()
        .zip(seq2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn main() {
    //read in fasta file
    // command needs to be: cargo run /filepath/test.txt

    // CHALLENGE: provide usage ergonomics for this
    // After you implement this, read about the `clap` crate
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //Initialize vectors to store headers and sequences
    let mut dna: Vec<String> = Vec::new();

    for line in contents.lines() {
        dna.push(line.to_string())
    }

    let total_mismatch = hamm(dna.first().unwrap(), dna.get(1).unwrap());

    println!("The hamming distance is {total_mismatch}");
}
