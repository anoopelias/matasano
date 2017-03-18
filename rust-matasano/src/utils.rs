use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rustc_serialize::base64::FromBase64;

pub fn from_base64_file(filename: &str) -> Vec<u8> {
    let file = File::open(filename).unwrap();
    let buf_file = BufReader::new(&file);
    let mut text = String::new();

    for line in buf_file.lines() {
        text.push_str(&line.unwrap());
    }

    text.as_str().from_base64().unwrap()
}
