pub trait Pkcs7Pad {
    fn pkcs7_pad(&self, size: u8) -> Self;
}

impl Pkcs7Pad for Vec<u8> {
    fn pkcs7_pad(&self, size: u8) -> Self {
        pad(&self, size)
    }
}

impl Pkcs7Pad for String {
    fn pkcs7_pad(&self, size: u8) -> Self {
        let padded_bytes = pad(&self.as_bytes(), size);
        String::from_utf8_lossy(&padded_bytes).into_owned()
    }
}

pub fn pad(bytes: &[u8], size: u8) -> Vec<u8> {
    let len = bytes.len() as u8;
    let pad_len = size - (len % size);
    let mut padded_bytes = bytes.to_vec();

    for _ in 0..pad_len {
        padded_bytes.push(pad_len as u8);
    }

    padded_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_one() {
        let input = &[0, 5, 7];
        let response = pad(input, 4);

        let expected = &[0, 5, 7, 1];
        assert_eq!(response.as_slice(), expected);
    }

    #[test]
    fn test_pad_five() {
        let input = &[0, 5, 7];
        let response = pad(input, 8);

        let expected = &[0, 5, 7, 5, 5, 5, 5, 5];
        assert_eq!(response.as_slice(), expected);
    }

    #[test]
    fn test_pad_longer_than_size() {
        let input = &[0, 5, 7, 9, 2, 12];
        let response = pad(input, 4);

        let expected = &[0, 5, 7, 9, 2, 12, 2, 2];
        assert_eq!(response.as_slice(), expected);
    }

    #[test]
    fn test_pad_full() {
        let input = &[0, 5, 7, 1];
        let response = pad(input, 4);

        let expected = &[0, 5, 7, 1, 4, 4, 4, 4];
        assert_eq!(response.as_slice(), expected);
    }

    #[test]
    fn test_pad_vector() {
        let input = vec![0, 5, 7];
        let response = input.pkcs7_pad(4);

        let expected = &[0, 5, 7, 1];
        assert_eq!(response.as_slice(), expected);
    }

    #[test]
    fn test_pad_string() {
        let input = String::from("YELLOW_SUBMARINE");
        let response = input.pkcs7_pad(20);

        let expected = "YELLOW_SUBMARINE\x04\x04\x04\x04";

        assert_eq!(response, expected);
    }

}

