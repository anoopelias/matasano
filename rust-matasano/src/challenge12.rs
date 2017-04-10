use rustc_serialize::base64::FromBase64;
use lib::oracle::Oracle;
use lib::analyzer;

pub fn run() {
    let input = String::from("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg") +
        &"aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq" +
        &"dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg" +
        &"YnkK";

    let input_bytes = input.as_str().from_base64().unwrap();
    let oracle = Oracle::new(Some(input_bytes));

    let keysize = find_keysize(&oracle);
    let plain_text = analyzer::analyze_ecb_oracle(&oracle, &keysize);

    println!("Challenge 12 :");
    println!("{}", plain_text);
}

fn find_keysize(oracle: &Oracle) -> usize {
    let mut input = String::from("A");
    let initial_size = oracle.encrypt(input.as_bytes()).len();

    loop {
        input.push_str("A");
        let output_size = oracle.encrypt(input.as_bytes()).len();

        if output_size != initial_size {
            return output_size - initial_size
        }
    }
}
