use lib::random::Random;
use lib::cryptor::Encryptor;
use lib::cryptor::Aes128EcbEncryptor;
use lib::cryptor::Aes128CbcEncryptor;

pub struct Oracle {
    random: Random,
    key: Vec<u8>,
    iv: Vec<u8>,
    unknown_bytes: Option<Vec<u8>>,
}

impl Oracle {

    pub fn new(unknown_bytes: Option<Vec<u8>>) -> Self {
        let mut random = Random::new();
        let key = &mut [0;16];
        let iv = &mut [0;16];

        random.fill_bytes(key);
        random.fill_bytes(iv);

        Oracle { random: random, key: key.to_vec(), iv: iv.to_vec(),
            unknown_bytes: unknown_bytes}
    }

    pub fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        match self.unknown_bytes {
            Some(ref unknown_bytes) => {
                let mut plain_bytes = Vec::from(bytes);
                plain_bytes.extend(unknown_bytes.iter().cloned());
                Aes128EcbEncryptor.encrypt(&plain_bytes, &self.key)
            },
            None => 
                Aes128EcbEncryptor.encrypt(bytes, &self.key),
        }
    }

    pub fn encrypt_random(&mut self, bytes: &[u8]) -> Vec<u8> {
        match self.random.rand() & 1 {
            0 => Aes128EcbEncryptor.encrypt(bytes, &self.key),
            _ => Aes128CbcEncryptor(&self.iv).encrypt(bytes, &self.key)
        }
    }

}
