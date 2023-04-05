use libs::utils;
use libs::cryptor::Decryptor;
use libs::cryptor::Aes128EcbDecryptor;

pub fn run() {
    let bytes = utils::from_base64_file("../resources/7.txt");
    let key = &String::from("YELLOW SUBMARINE").into_bytes();
    let decryptor = Aes128EcbDecryptor;

    let result = decryptor.decrypt(&bytes, key);
    println!("Challenge 7 : {}", String::from_utf8(result).unwrap());
}

