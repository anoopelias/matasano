use libs::cryptor::XorDecryptor;
use libs::utils;
use libs::keysize::find_optimum_keysize;

use challenge3;

pub fn run() {
    let bytes = utils::from_base64_file("../resources/6.txt");
    let keysize = find_optimum_keysize(&bytes);
    let lines = decrypt(&bytes, &keysize);

    println!("Challenge 6 : ");

    for line in lines {
        print!("{}", line);
    }
}

fn decrypt(bytes: &[u8], keysize: &usize) -> Vec<String> {
    let mut lines = Vec::new();
    let no_of_lines = (bytes.len() as f32 / *keysize as f32).ceil() as i32;

    for _ in 0..no_of_lines {
        lines.push(String::new());
    }

    for column_index in 0..*keysize {

        let mut column = Vec::new();

        for (i, b) in bytes.iter().enumerate() {
            if i % keysize == column_index {
                column.push(*b);
            }
        }

        let column_text = challenge3::decrypt(&column, XorDecryptor).1;

        for (i, c) in column_text.chars().enumerate() {
            lines[i].push(c);
        }
    }

    lines
}

