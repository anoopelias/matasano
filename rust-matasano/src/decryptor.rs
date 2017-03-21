use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use crypto::{ symmetriccipher, buffer, aes, blockmodes };

pub trait Decryptor {
    fn decrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8>;
}

pub struct XorDecryptor;

pub struct Aes128EcbDecryptor;

impl Decryptor for XorDecryptor {
    fn decrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8> {
        bytes.iter()
            .zip(key)
            .map(|(byte1, byte2)| byte1 ^ byte2)
            .collect::<Vec<u8>>()
    }
}

fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

impl Decryptor for Aes128EcbDecryptor {
    fn decrypt(&self, bytes: &[u8], key: &[u8]) -> Vec<u8> {
        decrypt(bytes, key).unwrap()
    }
}
