use lib::utils;
use lib::cryptor::Decryptor;
use lib::cryptor::Aes128CbcDecryptor;

pub fn run() {
    let bytes = utils::from_base64_file("../resources/10.txt");
    let key = &String::from("YELLOW SUBMARINE").into_bytes();
    let iv = &[0;16];
    let decryptor = Aes128CbcDecryptor(iv);

    let result = decryptor.decrypt(&bytes, key);
    println!("Challenge 10 : {}", String::from_utf8(result).unwrap());
}


