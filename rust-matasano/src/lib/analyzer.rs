use std::str::from_utf8;

use lib::pkcs7::Pkcs7Pad;
use lib::oracle::Oracle;

pub fn is_ecb(bytes: &[u8], keysize: &usize) -> bool {
    let mut left = bytes.chunks(*keysize).collect::<Vec<&[u8]>>();
    left.sort();

    // Shift and zip with itself to compare
    // consecutive values
    let mut right = left.to_vec(); // cloning
    right.remove(0);

    left.iter()
        .zip(right.iter())
        .any(|(left, right)| {
            left.iter().eq(right.iter())
        })
}

pub fn analyze_ecb_oracle(oracle: &Oracle, keysize: &usize) -> String {

    assert_ecb(oracle, keysize);

    let cipher_text_length = oracle.encrypt(&[]).len();
    let mut plain_bytes = Vec::new();

    for _ in 0..cipher_text_length {

        let cipher_len = (plain_bytes.len() / keysize + 1) * keysize;

        let prepend_str = get_prepend_string(plain_bytes.len(), keysize);
        let mut cipher_output = oracle.encrypt(prepend_str.as_bytes());
        cipher_output.truncate(cipher_len);

        let base_input = prepend_str + from_utf8(&plain_bytes).unwrap();
        for i in 0..255 {
            let ch = i as u8;
            let mut input = base_input.clone();
            input.push(ch as char);

            let mut cipher_last_char = oracle.encrypt(input.as_bytes());
            cipher_last_char.truncate(cipher_len);

            if cipher_last_char.iter().eq(cipher_output.iter()) {
                plain_bytes.push(ch);
                break;
            }
        }

    }

    String::from(from_utf8(&plain_bytes.pkcs7_unpad()).unwrap())
}

fn get_prepend_string(plain_bytes_len: usize, keysize: &usize) -> String {
    // Size of the input string should be one less than an exact
    // multiple of keysize
    let mut prepend_str = String::new();
    let prepend_len = keysize - (plain_bytes_len % keysize) - 1;
    for _ in 0..prepend_len {
        prepend_str.push('A');
    }

    prepend_str
}

fn assert_ecb(oracle: &Oracle, keysize: &usize) {
    let mut input = String::new();

    for _ in 0..(2 * keysize) {
        input.push('A');
    }

    let cipher_text = oracle.encrypt(input.as_bytes());
    if !is_ecb(&cipher_text, keysize) {
        panic!("Not ECB encryption");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ecb_positive() {
        assert!(is_ecb(&[1, 2, 3, 4, 3, 4, 5, 6], &(2_usize)));
    }

    #[test]
    fn test_is_ecb_negative() {
        assert!(!is_ecb(&[1, 2, 3, 4, 5, 6], &(2_usize)));
    }
}
