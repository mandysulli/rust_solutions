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

// Initialize index for iterating
let mut index = 0;
// Initialize rna string to add new to add characters back to
// need to get characters back to string to print in a normal way
let mut rna = String::new();

//Iterate through char vector and add A,C&G to string when found. If T found add U to string
while index < chars.len() {
        if chars[index] == 'A'{
            rna.push('A');
        } else if chars[index] == 'C'{
            rna.push('C');
        } else if chars[index] == 'G'{
            rna.push('G');
        } else if chars[index] == 'T'{
            rna.push('U');
        }
        index += 1;
    }
    
    println!("Your RNA sequence is:");
    println!("{}", rna);
}
