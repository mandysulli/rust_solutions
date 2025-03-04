use std::env;
use std::fs;

fn main() {
    //read in fasta file
    // command needs to be: cargo run /filepath/test.txt
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //Initialize vectors to store headers and sequences
    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();

    //Initialize counters and elemnet to hold sequences
    let mut header_counter = 0;
    let mut seq_counter = 0;
    let mut hold_seq = String::new();

    // Challenge: rewrite without two counters
    //Iterate through lines in contents
    for line in contents.lines() {
        //If line starts with '>' save to header vec and the increase counter
        if line.starts_with('>') {
            header.push(line.to_string());
            header_counter += 1;
        //If line  does not start with '>' move into seq handling
        } else {
            //If the sequence counter is less than the header counter then push first sequence to the seq vector
            //then increase the seq counter
            if seq_counter < header_counter {
                seq.push(line.to_string());
                seq_counter += 1;
                hold_seq = line.to_string().clone();
            //If the sequence counter is equal to the header counter then amend the current sequence onto the seq saved in vector
            //this takes the seqeunces from different lines and smushes them into one full seq
            } else if seq_counter == header_counter {
                let position: usize = (seq_counter - 1).try_into().unwrap();
                hold_seq.push_str(line);

                seq[position] = hold_seq.clone();
            }
        }
    }
    //check that sequence conversion works
    //println!("{:?}", seq);

    //Inialize vecotr for holding gc content
    let mut gc: Vec<f32> = Vec::new();
    //Iterate through seqeunces and count the occurance of G and C
    //Then divide that by the length of the sequence
    for i_seq in seq {
        gc.push(gc_content(&i_seq))
    }
    //check that the gc values are storing correctly
    //println!("{:?}", gc);

    //Inialize elemnets to hold the highest gc content and the header associated with highest gc content
    let mut max_gc: f32 = 0.0;
    let mut max_gc_header = "";

    //Interate through the gc contents stored in gc vector
    //when the a value is higher than the gc value being held in hold gc, store new value in hold_gc
    //and store associated header in hold name
    for (i, gc) in gc.iter().enumerate() {
        if *gc > max_gc {
            max_gc = *gc;
            max_gc_header = header[i].as_str();
        }
    }

    println!("{final_name}", final_name = max_gc_header.replace(">", ""));

    let final_gc = max_gc * 100.0;
    println!("{final_gc}");
}

fn gc_content(seq: &str) -> f32 {
    let mut gc_counter = 0;

    for j in seq.chars() {
        if j == 'G' || j == 'C' {
            gc_counter += 1;
        }
    }
    gc_counter as f32 / seq.len() as f32
}

fn _gc_content2(seq: &str) -> f32 {
    seq.chars()
        .filter(|base| *base == 'G' || *base == 'C')
        .count() as f32
        / seq.len() as f32
}
