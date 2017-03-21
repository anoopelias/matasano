use rustc_serialize::hex::FromHex;
use std::str;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;

use lib::decryptor::Decryptor;
use lib::decryptor::XorDecryptor;

// Data from http://www.data-compression.com/english.html 
const ENGLISH_CHAR_FREQ: &'static [(char, f32)] = &[
    ('a', 0.0651738),
    ('b', 0.0124248),
    ('c', 0.0217339),
    ('d', 0.0349835),
    ('e', 0.1041442),
    ('f', 0.0197881),
    ('g', 0.0158610),
    ('h', 0.0492888),
    ('i', 0.0558094),
    ('j', 0.0009033),
    ('k', 0.0050529),
    ('l', 0.0331490),
    ('m', 0.0202124),
    ('n', 0.0564513),
    ('o', 0.0596302),
    ('p', 0.0137645),
    ('q', 0.0008606),
    ('r', 0.0497563),
    ('s', 0.0515760),
    ('t', 0.0729357),
    ('u', 0.0225134),
    ('v', 0.0082903),
    ('w', 0.0171272),
    ('x', 0.0013692),
    ('y', 0.0145984),
    ('z', 0.0007836),
    (' ', 0.1918182),
];

pub fn run() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let (_, string) = decrypt_from_hex(input, XorDecryptor);

    println!("Challenge 3 : {}", string);
}

pub fn decrypt_from_hex<D: Decryptor>(input: &str, decryptor: D) -> (f32, String) {
   decrypt(&input.from_hex().unwrap(), decryptor)
}

pub fn decrypt<D: Decryptor>(input_bytes: &[u8], decryptor: D) -> (f32, String) {

    (0..255).fold((0f32, String::new()), |state, i| {
        let key_bytes = &vec![i;input_bytes.len()];
        let new_bytes = decryptor.decrypt(input_bytes, key_bytes);
        let (high_score, _) = state;

        match str::from_utf8(&new_bytes) {
            Ok(string) => {
                let score = english_score(string);
                if score > high_score {
                    (score, String::from(string))
                } else {
                    state
                }
            },
            _ => state
        }
    })
}

fn get_english_char_freq_map() -> HashMap<char, f32> {
    ENGLISH_CHAR_FREQ.iter().cloned().collect()
}

fn english_chars() -> Vec<char> {
    get_english_char_freq_map().keys().cloned().collect()
}

fn get_char_freqs(string: &str) -> HashMap<char, f32> {
    let mut char_map = HashMap::new(); 
    let len = string.len() as f32;
    let valid_chars = english_chars();

    for c in string.chars() {
        if valid_chars.contains(&c) {
            let val = char_map.entry(c).or_insert(0);
            *val += 1;
        }
    }

    char_map.into_iter().map(|(c, freq)| {
        (c, freq as f32 / len )
    }).collect::<HashMap<char, f32>>()

}

fn clean(string: &str) -> String {
    String::from(string).to_lowercase()
}

fn english_score(string: &str) -> f32 {
    let clean_string = clean(string);
    let string_freqs = get_char_freqs(clean_string.as_str());
    let english_freqs = get_english_char_freq_map();

    similarity(&string_freqs, &english_freqs)
}

fn similarity(map1: &HashMap<char, f32>, map2: &HashMap<char, f32>) -> f32 {

    // Counter cosine similarity
    let numerator = dot_product(map1, map2);
    let sum1 = sum_of_values(map1);
    let sum2 = sum_of_values(map2);

    numerator.sqrt() / (sum1 + sum2).sqrt()
}

fn dot_product<T>(map1: &HashMap<T, f32>, map2: &HashMap<T, f32>) -> f32
    where T: Eq + Hash + Debug {

    let keys = map1.keys().chain(map2.keys()).collect::<HashSet<&T>>();

    keys.iter().fold(0f32, |dot_product, key| {
        dot_product + (map1.get(key).unwrap_or(&0f32) * map2.get(key).unwrap_or(&0f32))
    })
}


fn sum_of_values<T>(map: &HashMap<T, f32>) -> f32
    where T: Eq + Hash {

    map.iter().fold(0f32, |sum, (_, &freq_per)| sum + freq_per)
}

#[test]
fn test_get_char_freqs() {
    let str = String::from("abcab");

    let char_freqs = get_char_freqs(&str);
    assert_eq!(*char_freqs.get(&'a').unwrap(), 0.4f32);
    assert_eq!(*char_freqs.get(&'c').unwrap(), 0.2f32);
}


#[test]
fn test_dot_product() {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    map1.insert('a', 1.0_f32);
    map1.insert('b', 2.0_f32);
    map1.insert('c', 3.0_f32);
    map1.insert('d', 3.0_f32);

    map2.insert('a', 1.0_f32);
    map2.insert('b', 2.0_f32);
    map2.insert('c', 3.0_f32);

    let product = dot_product(&map1, &map2);
    assert_eq!(product, 14.0_f32);
}

#[test]
fn test_dot_product_with_itself() {
    let mut map1 = HashMap::new();

    map1.insert('a', 1.0_f32);
    map1.insert('b', 2.0_f32);
    map1.insert('c', 3.0_f32);
    map1.insert('d', 4.0_f32);

    let product = dot_product(&map1, &map1);
    assert_eq!(product, 30.0_f32);
}
