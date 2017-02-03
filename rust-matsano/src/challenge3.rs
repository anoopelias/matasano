use rustc_serialize::hex::FromHex;
use std::str;
use std::collections::HashMap;

static ENGLISH_CHAR_FREQ: [char; 7] = ['e', 't', 'a', 'o', 'i', 'n', ' '];

pub fn run() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let input_bytes = input.from_hex().unwrap();

    for i in 0..150 {
        let xor_bytes = vec![i;input_bytes.len()];
        let new_bytes = xor(&input_bytes, xor_bytes);

        match str::from_utf8(&new_bytes){
            Ok(string) => score(string, &i),
            Err(_) => {}
        };

    }
}

fn score(string: &str, i: &u8) {
    let score = english_score(String::from(string));
    if score > 4 {
        println!("Challenge 3 : {} :: {}  :: {}", i, score, string);
    }
}

fn char_freq(string: &String) -> Vec<(char, i32)> {
    let mut char_map = HashMap::new(); 

    for c in string.chars() {
        let val = char_map.entry(c).or_insert(0);
        *val += 1;
    }

    let mut chars: Vec<(char, i32)> = char_map.into_iter().collect();

    chars.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    chars
}

fn clean(string: &String) -> String {
    string.to_lowercase()
}

fn english_score(string: String) -> i32 {
    let clean_string = clean(&string);
    let freq = char_freq(&clean_string);
    let length = ENGLISH_CHAR_FREQ.len();

    let top_freq = freq[0..length].to_vec();
    //println!("Freq : {:?}", top_freq);
    let score = top_freq.iter().fold(0, |score, &(ch, _)| {
        if ENGLISH_CHAR_FREQ.contains(&ch) {
            score + 1
        } else {
            score
        }
    });

    score
}

fn xor(ls1: &Vec<u8>, ls2: Vec<u8>) -> Vec<u8> {
    ls1.iter()
        .zip(ls2)
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect::<Vec<u8>>()
}
