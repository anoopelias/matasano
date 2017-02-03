use rustc_serialize::hex::FromHex;
use std::str;
use std::collections::HashMap;

pub fn run() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let input_bytes = input.from_hex().unwrap();

    for i in 88..89 {
        let xor_bytes = vec![i;input_bytes.len()];
        let new_bytes = xor(&input_bytes, xor_bytes);

        let new_str = match str::from_utf8(&new_bytes){
            Ok(string) => string,
            Err(_) => "abcd"
        };
        println!("i:{} : {:?}", i, new_str);

        english_score(new_str);
    }
}

fn char_freq(string: &str) -> Vec<(char, i32)> {
    let mut char_map = HashMap::new(); 

    for c in string.chars() {
        let val = char_map.entry(c).or_insert(0);
        *val += 1;
    }

    let mut chars: Vec<(char, i32)> = char_map.into_iter().collect();

    chars.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    chars
}

fn remove_spaces(string: &str) -> &str {

}

fn english_score(string: &str) -> i32 {
    let freq = char_freq(string);
    println!("Freq : {:?}", freq);

    if freq[0].0 == 'e' {
        println!("String : {:?}", string);
    }
    0
}

fn xor(ls1: &Vec<u8>, ls2: Vec<u8>) -> Vec<u8> {
    ls1.iter()
        .zip(ls2)
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect::<Vec<u8>>()
}
