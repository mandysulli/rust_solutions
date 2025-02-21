use std::env;
use std::fs;
use std::mem::replace;

fn main() {
    //read in fasta file
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    //Initialize vectors to store headers and sequences
    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();

    //Initialize counters and elemnet to hold sequences
    let mut header_counter: i32 = 0;
    let mut seq_counter: i32 = 0;
    let mut hold_seq: String = "".to_string();
    //Iterate through lines in contents
    for line in contents.lines() {
        //If line starts with '>' save to header vec and the increase counter
        if line.starts_with('>') {
            header.push(line.to_string());
            header_counter+=1;
        //If line  does not start with '>' move into seq handling
        } else {
            //If the sequence counter is less than the header counter then push first sequence to the seq vector
            //then increase the seq counter
            if seq_counter < header_counter {
                seq.push(line.to_string());
                seq_counter+=1;
                hold_seq = line.to_string().clone();
            //If the sequence counter is equal to the header counter then amend the current sequence onto the seq saved in vector
            //this takes the seqeunces from different lines and smushes them into one full seq
            } else if seq_counter == header_counter  {
                let position: usize = (seq_counter-1).try_into().unwrap();
                hold_seq.push_str(line);
                //It doesn't liek the way I use this but it lets it slide...
                std::mem::replace(&mut seq[position], hold_seq.clone());

            }
            
        }
    }
}
