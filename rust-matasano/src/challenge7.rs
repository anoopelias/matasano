use utils;

use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use crypto::{ symmetriccipher, buffer, aes, blockmodes };

pub fn run() {
    let bytes = utils::from_base64_file("../resources/7.txt");
    let key = &String::from("YELLOW SUBMARINE").into_bytes();

    let result = decrypt(&bytes, key);
    println!("Challenge 7 : {}", String::from_utf8(result.unwrap()).unwrap());
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
