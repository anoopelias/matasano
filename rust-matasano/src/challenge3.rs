use rustc_serialize::hex::FromHex;
use std::str;
use std::collections::HashMap;

static ENGLISH_CHAR_FREQ: [char; 13] = ['e', ' ', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u'];
static LENGTH :i32 = 13;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CharFreq {
    freq: i32,
    ch: char,
    rank: Option<i32>
}

impl CharFreq {
    fn new(ch: char, freq: i32) -> CharFreq {
        CharFreq {ch: ch, freq: freq, rank: None}
    }
}

pub fn run() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    find_string(input);
}

pub fn find_string(input: &str) {
    let input_bytes = input.from_hex().unwrap();

    for i in 0..255 {
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
    if score > 100 {
        println!("Challenge 3 : {} :: {}  :: {}", i, score, string);
    }
}

fn char_freq(string: &String) -> Vec<CharFreq> {
    let mut char_map = HashMap::new(); 

    for c in string.chars() {
        let val = char_map.entry(c).or_insert(CharFreq::new(c, 0));
        val.freq += 1;
    }

    let mut char_freqs: Vec<CharFreq> = char_map.into_iter()
        .map(|(_, char_freq)| char_freq)
        .collect();

    char_freqs.sort();
    char_freqs.reverse();

    char_freqs
}

fn clean(string: &String) -> String {
    string.to_lowercase()
}

fn rank(char_freqs: Vec<CharFreq>) -> Vec<CharFreq> {
    char_freqs.iter()
        .enumerate()
        .map(|(i, char_freq)| {
            CharFreq { rank: Some(i as i32), .. *char_freq }
        }).scan((0, -1), |prev, char_freq| {
            if prev.0 == char_freq.freq {
                Some(CharFreq {rank: Some(prev.1), .. char_freq})
            } else {
                prev.0 = char_freq.freq;
                prev.1 = char_freq.rank.unwrap();
                Some(char_freq)
            }
        }).collect::<Vec<CharFreq>>()
}

fn english_score(string: String) -> i32 {
    let clean_string = clean(&string);
    let freqs = rank(char_freq(&clean_string));
    let length = ENGLISH_CHAR_FREQ.len() as i32;

    freqs.iter()
        .filter(|&char_freq| {
            match char_freq.rank {
                Some(rank) if rank < length => true,
                _ => false
            }
        })
        .fold(0, |score, char_freq| {
            match ENGLISH_CHAR_FREQ.iter().position(|&r| r == char_freq.ch) {
                Some(position) => score + LENGTH - (position as i32  - char_freq.rank.unwrap()).abs(),
                None => score
            }
        })
}

fn xor(ls1: &Vec<u8>, ls2: Vec<u8>) -> Vec<u8> {
    ls1.iter()
        .zip(ls2)
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect::<Vec<u8>>()
}
