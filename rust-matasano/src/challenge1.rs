extern crate rustc_serialize;

use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::STANDARD;

pub fn run() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = hex_to_base64(input);

    println!("Challenge 1 : {}", output);
}

fn hex_to_base64(input: &str) -> String {
    let bytes = input.from_hex().unwrap();
    bytes.to_base64(STANDARD)
}
