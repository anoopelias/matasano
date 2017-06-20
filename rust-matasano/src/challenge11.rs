use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use lib::oracle::Oracle;
use lib::random::Random;
use lib::analyzer;

fn to_bytes(s: &str, random: &mut Random) -> Vec<u8> {
    let prefix_len = random.rand_range(&5, &10);
    let suffix_len = random.rand_range(&5, &10);

    let mut bytes = String::from(s).into_bytes().to_vec();

    for _ in 0..prefix_len {
        bytes.insert(1, 0);
    }

    for _ in 0..suffix_len {
        bytes.push(1);
    }

    bytes
}

pub fn run() {
    let mut cnt_ecb = 0;
    let mut cnt_cbc = 0;
    let mut random = Random::new();
    let mut input = String::new();

    let file = File::open("../resources/10_output.txt").unwrap();
    let mut buf_file = BufReader::new(&file);

    buf_file.read_to_string(&mut input).unwrap();

    for _ in 0..100 {

        let input_bytes = to_bytes(&input, &mut random);
        let mut oracle = Oracle::new();
        let cipher_bytes = oracle.encrypt_random(&input_bytes);

        match analyzer::is_ecb(&cipher_bytes, &16) {
            true => cnt_ecb = cnt_ecb + 1,
            false =>  cnt_cbc = cnt_cbc + 1
        }
    }

    println!("Challenge 11 : ECB : {}, CBC: {}", cnt_ecb, cnt_cbc);
}

