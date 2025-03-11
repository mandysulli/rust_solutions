use std::env;
use std::fs;

fn hamm(seq1: &str, seq2: &str) -> usize {
    let distance = seq1
        .chars()
        .zip(seq2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();

    return distance;
}

fn main() {
    //read in fasta file
    // command needs to be: cargo run /filepath/test.txt
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //Initialize vectors to store headers and sequences
    let mut dna: Vec<String> = Vec::new();

    for line in contents.lines() {
        dna.push(line.to_string())
    }

    let total_mismatch = hamm(dna.get(0).expect("REASON"), dna.get(1).expect("REASON"));

    println!("The hamming distance is {total_mismatch}");
}
