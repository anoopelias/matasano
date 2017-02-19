use rustc_serialize::hex::ToHex;

pub fn run() {
    let input = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = String::from("ICE");

    let input_bytes = input.into_bytes();
    let key_bytes = key.into_bytes();

    let output = key_bytes
        .iter()
        .cycle()
        .zip(input_bytes)
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect::<Vec<u8>>()
        .to_hex();

    println!("Challenge 5 : {}", output);
}

