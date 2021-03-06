pub trait Pkcs7Pad {
    fn pkcs7_pad(&self, size: u8) -> Self;
    fn pkcs7_unpad(&self) -> Result<Box<Self>, PaddingError>;
}

#[derive(Debug)]
pub enum PaddingError {
    IncorrectPadding
}

impl Pkcs7Pad for Vec<u8> {

    fn pkcs7_pad(&self, size: u8) -> Self {
        pad(&self, size)
    }

    fn pkcs7_unpad(&self) -> Result<Box<Self>, PaddingError> {
       unpad(&self).map(|v| Box::new(v))
    }
}

impl Pkcs7Pad for String {

    fn pkcs7_pad(&self, size: u8) -> Self {
        let padded_bytes = pad(&self.as_bytes(), size);
        String::from_utf8_lossy(&padded_bytes).into_owned()
    }

    fn pkcs7_unpad(&self) -> Result<Box<Self>, PaddingError> {
        unpad(&self.as_bytes()).map(|v| {
            Box::new(String::from_utf8_lossy(&v).into_owned())
        })
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

pub fn unpad(bytes: &[u8]) -> Result<Vec<u8>, PaddingError> {

    validate_padding(bytes)?;

    let len = bytes.len() as usize;
    let pad_len = bytes[len - 1];
    let mut unpadded_bytes = bytes.to_vec();

    for _ in 0..pad_len {
        unpadded_bytes.pop();
    }

    Ok(unpadded_bytes)
}

fn validate_padding(bytes: &[u8]) -> Result<(), PaddingError> {
    let len = bytes.len() as usize;
    let pad_len = bytes[len - 1];

    for i in 1..pad_len {
        if bytes[len - (i as usize) - 1] != pad_len {
            return Err(PaddingError::IncorrectPadding);
        }
    }

    Ok(())
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

    #[test]
    fn test_unpad_one() {
        let input = &[0, 5, 7, 1];
        let response = unpad(input);

        let expected = &[0, 5, 7];
        assert_eq!(response.unwrap().as_slice(), expected);
    }

    #[test]
    fn test_unpad_five() {
        let input = &[0, 5, 7, 5, 5, 5, 5, 5];
        let response = unpad(input);

        let expected = &[0, 5, 7];
        assert_eq!(response.unwrap().as_slice(), expected);
    }

    #[test]
    fn test_unpad_string() {
        let input = String::from("YELLOW_SUBMARINE\x04\x04\x04\x04");
        let response = input.pkcs7_unpad().unwrap();

        let expected = "YELLOW_SUBMARINE";
        assert_eq!(*response, expected);
    }

    #[test]
    fn test_unpad_error() {
        let input = String::from("YELLOW_SUBMARINE\x04\x04\x01\x04");
        assert!(input.pkcs7_unpad().is_err());
    }


}

