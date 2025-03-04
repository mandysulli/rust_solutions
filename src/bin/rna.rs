use std::io;

fn main() {
    let mut dna = String::new();

    //read in argument
    println!("Enter your DNA sequence below:");
    io::stdin()
        .read_line(&mut dna)
        .expect("Failed to read line");

    // This is a puzzle, so assume uppercase

    let rna: String = dna
        .chars()
        .map(|base| if base == 'T' { 'U' } else { base })
        .collect();

    //let rna = dna.replace("T", "U");

    /*
    // Moves DNA out. Now it is a Vec.
    let mut rna = dna.into_bytes();
    for base in rna.iter_mut() {
        if *base == b'T' {
            *base = b'U';
        }
    }
    */

    /*
    unsafe {
        for base in dna.as_bytes_mut() {
            if *base == b'T' {
                *base = b'U';
            }
        }
    }
    let rna = dna;
     */

    println!("Your RNA sequence is: {rna}");
    /*
    println!(
        "Your RNA sequence is: {rna}",
        rna = String::from_utf8_lossy(&rna)
    );
    */
}
