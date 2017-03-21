use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rustc_serialize::hex::FromHex;

pub fn run() {
    let file = File::open("../resources/8.txt").unwrap();
    let buf_file = BufReader::new(&file);

    for line in buf_file.lines() {
        let line_string = line.unwrap();
        if check_aes_ecb(&line_string) {
            println!("Challenge 8: {}", line_string);
        }
    }
    
}

pub fn check_aes_ecb(s: &str) -> bool {
    let bytes = s.from_hex().unwrap();
    let mut left = bytes.chunks(16).collect::<Vec<&[u8]>>();
    left.sort();

    // Shift and zip with itself to compare
    // consecutive values
    let mut right = left.to_vec(); // cloning
    right.remove(0);

    left.iter()
        .zip(right.iter())
        .any(|(left, right)| {
            left.iter().eq(right.iter())
        })
}

