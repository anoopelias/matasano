use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use challenge3;

pub fn run() {
    let file = File::open("../resources/4.txt").unwrap();
    let buf_file = BufReader::new(&file);

    let (_, text) = buf_file.lines().fold((0f32, String::new()), |state, line| {
        let new_state = challenge3::decrypt_from_hex(&line.unwrap());
        if new_state.0 > state.0 {
            new_state
        } else {
            state
        }
    });

    println!("Challenge 4 : {}", text);
}

