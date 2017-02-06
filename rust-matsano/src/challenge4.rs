use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use challenge3;

pub fn run() {
    let file = File::open("../resources/4.txt").unwrap();
    let buf_file = BufReader::new(&file);

    for line in buf_file.lines() {
        challenge3::find_string(&line.unwrap());
    }

}

