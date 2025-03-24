use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut my_string = String::new();

    // TODO: IF an argument is provided, read from that, otherwise read from the
    // stdin like below.
    if args.len() > 1 {
        let file_path = &args[1];
        my_string = fs::read_to_string(file_path).expect("Should have been able to read the file");
    } else {
        println!("Enter your DNA sequence below:");
        io::stdin()
            .read_line(&mut my_string)
            .expect("Failed to read line");
    }
    my_string.make_ascii_uppercase();

    let mut a_count = 0;
    let mut c_count = 0;
    let mut g_count = 0;
    let mut t_count = 0;

    for base in my_string.chars() {
        if base == 'A' {
            a_count += 1
        } else if base == 'C' {
            c_count += 1
        } else if base == 'G' {
            g_count += 1
        } else if base == 'T' {
            t_count += 1
        }
    }

    // I'm sorry but I altered the output from what Rosiland specificeied. Who
    // wouldn't want to know the nucleotide associated with the sum?
    println!("A:{a_count} C:{c_count} G:{g_count} T:{t_count}");
}
