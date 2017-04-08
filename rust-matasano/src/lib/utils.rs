use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rustc_serialize::base64::FromBase64;

pub fn from_base64_file(filename: &str) -> Vec<u8> {
    let file = File::open(filename).unwrap();
    let buf_file = BufReader::new(&file);
    let mut text = String::new();

    for line in buf_file.lines() {
        text.push_str(&line.unwrap());
    }

    text.as_str().from_base64().unwrap()
}

pub fn bits_in_num(num: &i32) -> i32 {
    match *num {
        0 => 0,
        _ => 1 + bits_in_num(&(num >> 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_bits_in_num() {
        assert_eq!(bits_in_num(&0), 0);
        assert_eq!(bits_in_num(&1), 1);
        assert_eq!(bits_in_num(&2), 2);
        assert_eq!(bits_in_num(&3), 2);
        assert_eq!(bits_in_num(&4), 3);
        assert_eq!(bits_in_num(&15), 4);
        assert_eq!(bits_in_num(&32), 6);
    }
}

