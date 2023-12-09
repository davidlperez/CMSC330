// To run via cargo from top-level project directory:
// # with command line arg 
// cargo run --quiet --bin probx_wc -- gettysburg.txt 
// # no command line arg
// cargo run --quiet --bin probx_wc -- 

use std::process::exit;         // bare use of exit(num) function to exit program
use std::env::args;             // bare use of args() function to retrieve commandline args
use std::fs::File;              // for file io
use std::io::{prelude::*, BufReader}; 

// Above import does two things
// 1. Imports everying in std::io::prelude to give access to common IO
//    traits
// 2. Imports BufReader struct which implements line-by-line input
//    processing


// YOUR CODE BELOW for implementing the word count program
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <filename>", args[0]);
        exit(1);
    }

    let filename = &args[1];

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Couldn't open file {}: {}", filename, error);
            exit(1);
        }
    };

    let reader: BufReader<File> = BufReader::new(file);

    let mut line_count: i32 = 0;
    let mut word_count: i32 = 0;
    let mut char_count: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        line_count += 1;
        word_count += count_words(&line);
        char_count += line.len() as i32 + 1; // +1 for the newline character
    }

    println!("{:4} {:4} {:4} {}", line_count, word_count, char_count, filename);
}

fn count_words(text: &str) -> i32 {
    text.split_whitespace().count() as i32
}