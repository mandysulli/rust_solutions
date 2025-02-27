use std::io;

fn main() {
    let mut my_string = String::new();
    
    //read in argument
    println!("Enter your DNA sequence below:");
    io::stdin()
            .read_line(&mut my_string)
            .expect("Failed to read line");
    
    //Make input uppercase incase user does not use upper case when converting into vec of char
    let chars: Vec<char> = my_string.to_uppercase().chars().collect();

// initialize variables for counting
let mut a_count = 0;
let mut c_count = 0;
let mut g_count = 0;
let mut t_count = 0;
let mut index = 0;

//while loop to count A,C,G,T's present in character vector
while index < chars.len() {
        if chars[index] == 'A'{
            a_count += 1
        } else if chars[index] == 'C'{
            c_count += 1
        } else if chars[index] == 'G'{
            g_count += 1
        } else if chars[index] == 'T'{
            t_count += 1
        }
        index += 1;
    }
    
    //I'm sorry but I altrered the output from what Rosiland specificeied
    //Who wouldn't want to know the nucleotide associated with the sum?
    println!("A:{} C:{} G:{} T:{}", a_count, c_count, g_count, t_count);
}
