use std::str::from_utf8;

use lib::pkcs7::Pkcs7Pad;
use lib::oracle::Oracle;

pub struct OracleAnalyzer {
    oracle: Oracle,
}

impl OracleAnalyzer {
    pub fn new(oracle: Oracle) -> OracleAnalyzer {
        OracleAnalyzer { oracle: oracle }
    }

    pub fn analyze_ecb(&self) -> Result<String, AnalyzerError> {
        let keysize = self.find_keysize();
        self.assert_ecb(&keysize)?;
        let prefix_len = self.find_prefix_len(&keysize);

        // OracleWrapper will take care of the prefix
        let oracle_wrapper = 
            OracleWrapper::new(&self.oracle, &keysize, prefix_len);

        let cipher_bytes = oracle_wrapper.encrypt(&[]);
        let cipher_bytes_len = cipher_bytes.len();
        let mut plain_bytes = Vec::new();

        for _ in 0..cipher_bytes_len {
            let next_byte = oracle_wrapper.ecb_next_byte(&plain_bytes);

            // ecb_next_byte logic will start failing after it has found the
            // first byte of the padding
            match next_byte {
                Some(byte) => plain_bytes.push(byte),
                None => break
            }
        }

        self.fix_padding(&mut plain_bytes, &keysize, &cipher_bytes_len);

        //Final validation
        if !oracle_wrapper.encrypt(&plain_bytes)
            .iter()
            .take(cipher_bytes.len())
            .eq(cipher_bytes.iter()) {
                Err(AnalyzerError::UnknownError)?;
            }

        from_utf8(&plain_bytes.pkcs7_unpad().expect("Unpad Error"))
            .map(|plain_text| {
                String::from(plain_text)
            }).map_err(|_| {
                AnalyzerError::UnknownError
            })
    }

    fn fix_padding(&self, plain_bytes: &mut Vec<u8>, keysize: &usize,
        cipher_text_len: &usize) {

        // The top element in plain_bytes will be '1' since from next
        // padding onwards, ecb_next_byte function on OracleWrapper will
        // fail
        let top = plain_bytes.pop().unwrap();
        assert_eq!(top, 1);

        let pad_len = cipher_text_len - plain_bytes.len();
        assert!(pad_len <= *keysize);

        for _ in 0..pad_len {
            plain_bytes.push(pad_len as u8);
        }
    }

    fn find_prefix_len(&self, keysize: &usize) -> usize {
        let prefix_block_id = self.find_prefix_block_id(keysize);
        let prefix_range = prefix_block_id * keysize..
            (prefix_block_id + 1) * keysize;
        let mut input_str = String::new();
        let mut prev_prefix_block: Option<Vec<u8>> = None;

        // prefix_block is the ending block of the prefix.
        // Here, we are trying to find the length of prefix part in this block.
        loop {
            let cipher_bytes = self.oracle.encrypt(input_str.as_bytes());
            let prefix_block = &cipher_bytes[prefix_range.clone()].to_vec();

            if prev_prefix_block.is_some() &&
                prev_prefix_block.unwrap().eq(prefix_block) {

                break;
            }

            input_str.push('A');
            prev_prefix_block = Some(prefix_block.to_vec());
        }

        ((prefix_block_id + 1) * keysize)  - (input_str.len() - 1)

    }

    fn find_prefix_block_id(&self, keysize: &usize) -> usize {
        let cipher_a = self.oracle.encrypt("A".as_bytes());
        let cipher_b = self.oracle.encrypt("B".as_bytes());

        let zip_list = cipher_a.chunks(keysize.clone()).zip(
            cipher_b.chunks(keysize.clone()));

        // First mismatching block will be the prefix block
        zip_list.enumerate()
            .find(|&(_, (block1, block2))| !block1.iter().eq(block2.iter()))
            .map(|(prefix_block, (_, _))| prefix_block)
            .unwrap()
    }

    fn find_keysize(&self) -> usize {
        let mut input = String::from("A");
        let initial_size = self.oracle.encrypt(input.as_bytes()).len();
        let keysize;

        loop {
            input.push('A');
            let output_size = self.oracle.encrypt(input.as_bytes()).len();

            if output_size != initial_size {
                keysize = output_size - initial_size;
                break;
            }
        }

        keysize
    }

    fn assert_ecb(&self, keysize: &usize) -> Result<(), AnalyzerError> {
        let mut input = String::new();

        // The oracle could be both prefixing and suffixing the plain text.
        // So only three times the keysize will gurantee an ECB detection.
        for _ in 0..(3 * keysize) {
            input.push('A');
        }

        let cipher_text = self.oracle.encrypt(input.as_bytes());

        if !is_ecb(&cipher_text, keysize) {
            Err(AnalyzerError::NotEcb)
        } else {
            Ok(())
        }
    }

}

struct OracleWrapper<'a> {
    oracle: &'a Oracle,
    prefix_fill: Vec<u8>,
    prefix_block_len: usize,
    keysize: usize,
}

impl<'a> OracleWrapper<'a> {
    fn new(oracle: &'a Oracle, keysize: &usize, prefix_len: usize)
        -> OracleWrapper<'a> {

        let mut prefix_fill = Vec::new();
        let mut prefix_fill_len = keysize - (prefix_len % keysize);

        if prefix_fill_len == *keysize {
            prefix_fill_len = 0
        }

        for _ in 0..prefix_fill_len {
            prefix_fill.push(32);
        }

        OracleWrapper { oracle: oracle, prefix_fill: prefix_fill,
            keysize: *keysize, prefix_block_len: (prefix_len + prefix_fill_len) }
    }

    fn encrypt(&self, plain_bytes: &[u8]) -> Vec<u8> {
        let mut input = self.prefix_fill.clone();
        input.extend(plain_bytes);

        self.oracle.encrypt(&input)
            .iter()
            .skip(self.prefix_block_len)
            .cloned()
            .collect()
    }

    fn encrypt_take(&self, plain_bytes: &[u8], len: &usize) -> Vec<u8>{
        self.encrypt(plain_bytes)
            .iter()
            .take(*len)
            .cloned()
            .collect()
    }

    fn ecb_next_byte(&self, plain_bytes: &[u8]) -> Option<u8> {
        let plain_bytes_len = plain_bytes.len();

        // Prepare a prepend vector such that the size of the input string
        // is exactly one less than an exact multiple of keysize
        let prepend_len = self.keysize - (plain_bytes_len % self.keysize) - 1;
        let mut prepend_bytes = Vec::new();
        for _ in 0..prepend_len {
            prepend_bytes.push('A' as u8);
        }

        // Length of the cipher text under consideration
        let cipher_len = prepend_len + plain_bytes_len + 1;
        let cipher_output = self.encrypt_take(&prepend_bytes, &cipher_len);

        prepend_bytes.extend(plain_bytes);

        (0..255).find(|byte| {
            let mut input = prepend_bytes.clone();
            input.push(*byte as u8);

            self.encrypt_take(&input, &cipher_len)
                .iter()
                .eq(cipher_output.iter())
        })
    }
}

#[derive(Debug)]
pub enum AnalyzerError {
    NotEcb,
    UnknownError
}

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

    #[test]
    fn test_analyze_ecb() {
        let input = "foobar";
        let oracle = Oracle::new(None, Some(input.as_bytes().to_vec()));
        let oracle_analyzer = OracleAnalyzer::new(oracle);

        let output = oracle_analyzer.analyze_ecb().unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn test_analyze_ecb_keysize() {
        let input = "PURPLE SPEEDBOAT";
        let oracle = Oracle::new(None, Some(input.as_bytes().to_vec()));
        let oracle_analyzer = OracleAnalyzer::new(oracle);

        let output = oracle_analyzer.analyze_ecb().unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn test_analyze_ecb_less_than_keysize() {
        let input = "PURPLE SPEEDBOA";
        let oracle = Oracle::new(None, Some(input.as_bytes().to_vec()));
        let oracle_analyzer = OracleAnalyzer::new(oracle);

        let output = oracle_analyzer.analyze_ecb().unwrap();
        assert_eq!(output, input);
    }

}
