use crypto::buffer::{ ReadBuffer, WriteBuffer, RefReadBuffer, RefWriteBuffer,
    BufferResult};
use crypto::buffer::BufferResult::{ BufferUnderflow, BufferOverflow};
use crypto::aes::{ ecb_decryptor, ecb_encryptor, KeySize};
use crypto::blockmodes::{ PkcsPadding, NoPadding, PaddingProcessor};
use crypto::symmetriccipher;
use crypto::symmetriccipher::SymmetricCipherError;

use libs::pkcs7::Pkcs7Pad;

trait CryptHandler {
    fn crypt(&mut self, read_buffer: &mut RefReadBuffer,
         write_buffer: &mut RefWriteBuffer)
        -> Result<BufferResult, SymmetricCipherError>;
}

impl CryptHandler for Box<symmetriccipher::Encryptor> {
    fn crypt(&mut self, read_buffer: &mut RefReadBuffer,
         write_buffer: &mut RefWriteBuffer)
        -> Result<BufferResult, SymmetricCipherError> {

        self.encrypt(read_buffer, write_buffer, true)
    }
}

impl CryptHandler for Box<symmetriccipher::Decryptor> {
    fn crypt(&mut self, read_buffer: &mut RefReadBuffer,
         write_buffer: &mut RefWriteBuffer)
        -> Result<BufferResult, SymmetricCipherError> {

        self.decrypt(read_buffer, write_buffer, true)
    }
}

fn crypt<C: CryptHandler>(bytes: &[u8], mut crypt_handler: C)
    -> Result<Vec<u8>, SymmetricCipherError> {

    let mut final_result = Vec::<u8>::new();
    let mut buffer = [0; 4096];
    let mut read_buffer = &mut RefReadBuffer::new(bytes);
    let mut write_buffer = &mut RefWriteBuffer::new(&mut buffer);

    loop {
        let result = crypt_handler.crypt(read_buffer, write_buffer)?;
        final_result.extend(write_buffer
            .take_read_buffer()
            .take_remaining()
            .iter()
            .map(|&i| i));

        match result {
            BufferUnderflow => break,
            BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes.iter()
        .zip(key)
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect::<Vec<u8>>()
}

fn decrypt<P>(cipher_bytes: &[u8], key: &[u8], padding: P)
    -> Result<Vec<u8>, SymmetricCipherError> 
    where P: PaddingProcessor + Send + 'static {

    let decryptor = ecb_decryptor(KeySize::KeySize128, key, padding);
    crypt(cipher_bytes, decryptor)
}

fn encrypt<P>(plain_bytes: &[u8], key: &[u8], padding: P)
    -> Result<Vec<u8>, SymmetricCipherError>
    where P: PaddingProcessor + Send + 'static {

    let encryptor = ecb_encryptor(KeySize::KeySize128, key, padding);
    crypt(plain_bytes, encryptor)
}

pub trait Decryptor {
    fn decrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8>;
}

pub trait Encryptor {
    fn encrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8>;
}

pub struct XorDecryptor;
pub struct Aes128EcbDecryptor;
pub struct Aes128EcbEncryptor;
pub struct Aes128CbcEncryptor<'a>(pub &'a [u8]);
pub struct Aes128CbcDecryptor<'a>(pub &'a [u8]);

impl Decryptor for XorDecryptor {
    fn decrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8> {
        xor(bytes, key)
    }
}

impl Decryptor for Aes128EcbDecryptor {
    fn decrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8> {
        decrypt(bytes, key, PkcsPadding).unwrap()
    }
}

impl Encryptor for Aes128EcbEncryptor {
    fn encrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8> {
        encrypt(bytes, key, PkcsPadding).unwrap()
    }
}

impl<'a> Encryptor for Aes128CbcEncryptor<'a> {
    fn encrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8> {
        let iv = Vec::from(self.0);

        bytes.to_vec()
            .pkcs7_pad(16)
            .chunks(16)
            .scan(iv, |prev, chunk| {
                let xor_bytes = xor(prev, chunk);
                let cipher_chunk = encrypt(xor_bytes.as_slice(), key, NoPadding)
                    .unwrap();

                prev.clear();
                prev.extend(cipher_chunk.iter().cloned());

                Some(cipher_chunk)
            }).flat_map(|cipher_chunk| cipher_chunk)
            .collect()

    }
}

impl<'a> Decryptor for Aes128CbcDecryptor<'a> {
    fn decrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8> {
        let iv = Vec::from(self.0);

        // Dereference operator needed here to unbox the unpad
        // operation
        *bytes.to_vec()
            .chunks(16)
            .scan(iv, |prev, chunk| {
                let cipher_chunk = decrypt(chunk, key, NoPadding).unwrap();
                let plain_chunk = xor(prev, &cipher_chunk);

                prev.clear();
                prev.extend(chunk.iter().cloned());

                Some(plain_chunk)
            })
            .flat_map(|cipher_chunk| cipher_chunk)
            .collect::<Vec<u8>>()
            .pkcs7_unpad()
            .expect("Padding Error")

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_ecb_encrypt() {
        let plain_bytes = "foo".as_bytes();
        let key = "YELLOW SUBMARINE".as_bytes();

        let cipher_bytes = Aes128EcbEncryptor.encrypt(plain_bytes, key);

        // https://goo.gl/QMFvYv 
        let expected = &[94, 162, 90, 181, 151, 215, 195, 200, 101, 224, 126,
            168, 205, 179, 168, 166];
        assert_eq!(cipher_bytes.as_slice(), expected);
    }

    #[test]
    fn test_aes_ecb_decrypt() {
        let cipher_bytes = &[94, 162, 90, 181, 151, 215, 195, 200, 101, 224,
            126, 168, 205, 179, 168, 166];
        let key = "YELLOW SUBMARINE".as_bytes();

        let plain_bytes = Aes128EcbDecryptor.decrypt(cipher_bytes, key);
        let expected = "foo".as_bytes();
        assert_eq!(plain_bytes.as_slice(), expected);
    }

    #[test]
    fn test_aes_cbc_encrypt() {
        let plain_bytes = "PURPLE SPEEDBOAT".as_bytes();
        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = "GREEN SPACECRAFT".as_bytes();

        let cipher_bytes = Aes128CbcEncryptor(iv).encrypt(plain_bytes, key);

        // https://goo.gl/sB3U12
        let expected = &[66, 19, 152, 42, 202, 25, 162, 144, 39, 160, 93, 255,
            229, 173, 214, 164, 254, 151, 189, 40, 240, 44, 51, 234, 47, 184,
            138, 134, 68, 216, 84, 28];
        assert_eq!(cipher_bytes.as_slice(), expected);
    }

    #[test]
    fn test_aes_cbc_encrypt_message_lt_128bit() {
        let plain_bytes = "foo".as_bytes();
        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = "GREEN SPACECRAFT".as_bytes();

        let cipher_bytes = Aes128CbcEncryptor(iv).encrypt(plain_bytes, key);

        // https://goo.gl/WUt5br
        let expected = &[118, 250, 12, 228, 185, 89, 251, 169, 77, 66, 236,
            123, 185, 11, 6, 134];
        assert_eq!(cipher_bytes.as_slice(), expected);
    }

    #[test]
    fn test_aes_cbc_encrypt_message_gt_128bit() {
        let plain_bytes = "PURPLE SPEEDBOAT foo".as_bytes();
        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = "GREEN SPACECRAFT".as_bytes();

        let cipher_bytes = Aes128CbcEncryptor(iv).encrypt(plain_bytes, key);

        // https://goo.gl/XbPq6B
        let expected = &[66, 19, 152, 42, 202, 25, 162, 144, 39, 160, 93, 255,
            229, 173, 214, 164, 55, 144, 240, 224, 150, 108, 176, 142, 120,
            216, 30, 186, 51, 210, 88, 129];
        assert_eq!(cipher_bytes.as_slice(), expected);
    }

    #[test]
    fn test_aes_cbc_decrypt() {
        let cipher_bytes = &[66, 19, 152, 42, 202, 25, 162, 144, 39, 160, 93, 255,
            229, 173, 214, 164, 254, 151, 189, 40, 240, 44, 51, 234, 47, 184,
            138, 134, 68, 216, 84, 28];

        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = "GREEN SPACECRAFT".as_bytes();

        let plain_bytes = Aes128CbcDecryptor(iv).decrypt(cipher_bytes, key);

        let expected = "PURPLE SPEEDBOAT".as_bytes();
        assert_eq!(plain_bytes.as_slice(), expected);
    }

    #[test]
    fn test_aes_cbc_decrypt_message_lt_128bit() {
        let cipher_bytes = &[118, 250, 12, 228, 185, 89, 251, 169, 77, 66, 236,
            123, 185, 11, 6, 134];
        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = "GREEN SPACECRAFT".as_bytes();

        let plain_bytes = Aes128CbcDecryptor(iv).decrypt(cipher_bytes, key);

        let expected = "foo".as_bytes();
        assert_eq!(plain_bytes.as_slice(), expected);
    }

    #[test]
    fn test_aes_cbc_decrypt_message_gt_128bit() {
        let cipher_bytes = &[66, 19, 152, 42, 202, 25, 162, 144, 39, 160, 93, 255,
            229, 173, 214, 164, 55, 144, 240, 224, 150, 108, 176, 142, 120,
            216, 30, 186, 51, 210, 88, 129];
        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = "GREEN SPACECRAFT".as_bytes();

        let plain_bytes = Aes128CbcDecryptor(iv).decrypt(cipher_bytes, key);

        let expected = "PURPLE SPEEDBOAT foo".as_bytes();
        assert_eq!(plain_bytes.as_slice(), expected);
    }

}
