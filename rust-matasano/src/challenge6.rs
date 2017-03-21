use lib::decryptor::XorDecryptor;
use lib::utils;
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

fn find_optimum_keysize(bytes: &[u8]) -> usize {
    let (_, optimum_keysize) = (2..40).fold((20f32, 0usize), |state, keysize| {
        let dist = avg_norm_dist(&bytes, keysize);
        let (min_dist, _) = state;

        if dist < min_dist {
            (dist, keysize)
        } else {
            state
        }
    });

    optimum_keysize
}

fn avg_norm_dist(bytes: &[u8], keysize: usize) -> f32 {
    let no_of_sets = bytes.len() / keysize;
    let mut sets = vec![];

    for i in 0..no_of_sets {
        let start = i * keysize;
        sets.push(&bytes[start..(start + keysize)]);
    }

    let sum = sets.iter()
        .zip(&sets[1..no_of_sets])
        .fold(0f32, |sum, (set1, set2)| {
            sum + norm_dist(set1, set2, keysize)
        });

    sum / (no_of_sets - 1) as f32
}

fn norm_dist(bytes1: &[u8], bytes2: &[u8], len: usize) -> f32 {
    hamming_dist(&bytes1, &bytes2) as f32 / len as f32
}

fn hamming_dist(inp1: &[u8], inp2: &[u8]) -> i32 {
    inp1.iter()
        .zip(inp2)
        .map(|(b1, b2)| b1 ^ b2)
        .fold(0, |dist, b| {
            dist + count_bits(b) as i32
        })
}

fn count_bits(n: u8) -> u8 {
    match n {
        0 => 0,
        n => (n & 1) + count_bits(n >> 1)
    }
}

#[test]
fn test_average_norm_dist() {
    let bytes = &[1, 3, 5, 7, 5, 7, 10, 9];
    let keysize = 2usize;
    assert_eq!(avg_norm_dist(bytes, keysize), 1.5f32);
}

#[test]
fn test_norm_dist() {
    let bytes1 = &[1, 3];
    let bytes2 = &[5, 7];
    let keysize = 2usize;
    assert_eq!(norm_dist(bytes1, bytes2, keysize), 1.0f32);
}

#[test]
fn test_norm_dist_with_longer_string() {
    let bytes1 = &[1, 3, 5, 7];
    let bytes2 = &[5, 7, 7, 8];
    let keysize = 4usize;
    assert_eq!(norm_dist(bytes1, bytes2, keysize), 1.75f32);
}

#[test]
fn test_count_bits() {
    assert_eq!(count_bits(5), 2);
    assert_eq!(count_bits(7), 3);
}


#[test]
fn test_hamming_dist() {
    let inp1 = &[0, 0];
    let inp2 = &[1, 1];

    assert_eq!(hamming_dist(inp1, inp2), 2);
}

#[test]
fn test_hamming_dist_string() {
    let inp1 = String::from("this is a test").into_bytes();
    let inp2 = String::from("wokka wokka!!!").into_bytes();

    assert_eq!(hamming_dist(&inp1, &inp2), 37);
}

