use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_calories_input() -> Vec<String> {
    let mut vec = vec![];
    let file = File::open("input.txt").expect("did not work");    
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        vec.push(line.unwrap());
    }

    vec
}
