use std::io;

fn main() {
    let mut forward = String::new();

    //read in argument
    println!("Enter your DNA sequence below:");
    io::stdin()
        .read_line(&mut forward)
        .expect("Failed to read line");

    let revcomp: String = forward.trim_end().chars().rev().map(complement).collect();

    println!("The reverse compliment sequence is:\n{revcomp}");
}

fn complement(base: char) -> char {
    match base.to_ascii_uppercase() {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => base,
    }
}
