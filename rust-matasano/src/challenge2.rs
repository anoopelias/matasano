use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;

pub fn run() {
    let (input_str1, input_str2) = (
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965");

    let hex1 = input_str1.from_hex().unwrap();
    let hex2 = input_str2.from_hex().unwrap();

    let output = hex1
        .iter()
        .zip(hex2)
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect::<Vec<u8>>()
        .to_hex();

    println!("Challenge 2 : {}", output);
}
