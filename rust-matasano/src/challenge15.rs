use lib::pkcs7::Pkcs7Pad;

pub fn run() {
    let inputs = vec![
        String::from("ICE ICE BABY\x04\x04\x04\x04"),
        String::from("ICE ICE BABY\x05\x05\x05\x05"),
        String::from("ICE ICE BABY\x01\x02\x03\x04"),
    ];

    let output = inputs.iter().map(|input| {
        match input.pkcs7_unpad() {
            Ok(text) => *text,
            Err(_) => String::from("Unpad Error"),
        }

    }).collect::<Vec<String>>()
    .join("\n");

    print!("Challenge 15: ");
    print!("{}", output);
    println!("");

}

