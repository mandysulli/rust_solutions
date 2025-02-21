use std::env;
use std::fs;
use std::mem::replace;

fn main() {
    //read in fasta file
    // command needs to be: cargo run /filepath/test.txt
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
    //check that sequence conversion works
    //println!("{:?}", seq);

    //Inialize vecotr for holding gc content
    let mut gc: Vec<f32> = Vec::new();
    //Iterate through seqeunces and count the occurance of G and C
    //Then divide that by the length of the sequence
    for i_seq in seq {
        let mut gc_counter = 0;
        let seq_length=i_seq.len();
        for j in i_seq.chars() {
            if j == 'G'{
                gc_counter += 1;
            } else if j == 'C' {
                gc_counter += 1;
            }
        }
        let mut gc_content: f32= gc_counter as f32/seq_length as f32;
        gc.push(gc_content)
        
    }
    //check that the gc values are storing correctly
    //println!("{:?}", gc);

    //Inialize elemnets to hold the highest gc content and the header associated with highest gc content
    let mut hold_gc: f32 = 0.0;
    let mut hold_name = &String::new();
    let mut index = 0;
    //Interate through the gc contents stored in gc vector
    //when the a value is higher than the gc value being held in hold gc, store new value in hold_gc 
    //and store associated header in hold name
    for i in gc {
        if i > hold_gc {
            hold_gc = i.clone();
            hold_name = header.get(index).unwrap();
        } else {
            continue
        }
        index+=1;
    }

    //Convert outputs to specified format and print
    let final_name = hold_name.replace(">", "");
    let final_gc = hold_gc*100.0;
    println!("{}", final_name);
    println!("{}", final_gc);
}
