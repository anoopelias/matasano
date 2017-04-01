use lib::pkcs7::Pkcs7Pad;

pub fn run() {
    let input = String::from("YELLOW SUBMARINE");
    let output = input.pkcs7_pad(20);

    println!("Challenge 9: {}", output);
}

