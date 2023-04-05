use rustc_serialize::base64::FromBase64;
use libs::oracle::Oracle;
use libs::analyzer::OracleAnalyzer;

pub fn run() {
    let input = String::from("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg") +
        &"aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq" +
        &"dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg" +
        &"YnkK";

    let input_bytes = input.as_str().from_base64().unwrap();
    let oracle = Oracle::new(None, Some(input_bytes));
    let oracle_analyzer = OracleAnalyzer::new(oracle);

    let plain_text = oracle_analyzer.analyze_ecb()
        .expect("Error while analyzing ECB");

    println!("Challenge 12 :");
    println!("{}", plain_text);
}
