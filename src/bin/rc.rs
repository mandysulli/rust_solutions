use std::io;

fn main() {
    let mut my_string = String::new();

    //read in argument
    println!("Enter your DNA sequence below:");
    io::stdin()
        .read_line(&mut my_string)
        .expect("Failed to read line");

    //Make input uppercase incase user does not use upper case
    let chars: Vec<char> = my_string.to_uppercase().chars().collect();

    // Initialize rev comp string to add new to add characters back to
    let mut rc = String::new();
    //Initialize bad_nuc for error tracking
    let mut bad_nuc = 0;

    //Iterate through char vector in the reverse order
    //Add opposite nucleotide to the rc string than one found in char vec
    for i in (0..chars.len()).rev() {
        if chars[i] == 'A' {
            rc.push('T');
        } else if chars[i] == 'C' {
            rc.push('G');
        } else if chars[i] == 'G' {
            rc.push('C');
        } else if chars[i] == 'T' {
            rc.push('A');
        }
    }

    println!("The reverse compliment sequence is:");
    println!("{}", rc);
}

//wanted to add error handling, but it went off the rails
//Ask Sam about it if can't figure out later
