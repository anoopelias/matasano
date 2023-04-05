use libs::random::Random;
use libs::cryptor::Encryptor;
use libs::cryptor::Decryptor;
use libs::cryptor::Aes128EcbEncryptor;
use libs::cryptor::Aes128EcbDecryptor;
use libs::cryptor::Aes128CbcEncryptor;
use libs::cryptor::Aes128CbcDecryptor;

pub struct Oracle {
    random: Random,
    key: Vec<u8>,
    iv: Vec<u8>,
    prefix: Option<Vec<u8>>,
    suffix: Option<Vec<u8>>,
}

impl Oracle {

    pub fn new(prefix: Option<Vec<u8>>, suffix: Option<Vec<u8>>) -> Self {
        Oracle::new_oracle(prefix, suffix, Random::new())
    }

    pub fn random_prefix(suffix: Option<Vec<u8>>) -> Self {
        let mut random = Random::new();
        Oracle::new_oracle(Some(random.rand_bytes(&256)), suffix, random)
    }

    fn new_oracle(prefix: Option<Vec<u8>>, suffix: Option<Vec<u8>>,
        mut random: Random) -> Self {

        let key = &mut [0;16];
        let iv = &mut [0;16];

        random.fill_bytes(key);
        random.fill_bytes(iv);

        Oracle { random: random, key: key.to_vec(), iv: iv.to_vec(),
            prefix: prefix, suffix: suffix}
    }

    fn get_plain_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let blank_slice = Vec::new();
        let mut plain_bytes = Vec::new();

        let prefix = self.prefix.as_ref().unwrap_or(&blank_slice);
        let suffix = self.suffix.as_ref().unwrap_or(&blank_slice);

        plain_bytes.extend(prefix);
        plain_bytes.extend(bytes);
        plain_bytes.extend(suffix);

        plain_bytes
    }

    pub fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let plain_bytes = self.get_plain_bytes(bytes);
        Aes128EcbEncryptor.encrypt(&plain_bytes, &self.key)
    }

    pub fn decrypt(&self, bytes: &[u8]) -> Vec<u8> {
        Aes128EcbDecryptor.decrypt(bytes, &self.key)
    }

    pub fn encrypt_random(&mut self, bytes: &[u8]) -> Vec<u8> {
        match self.random.rand() & 1 {
            0 => Aes128EcbEncryptor.encrypt(bytes, &self.key),
            _ => Aes128CbcEncryptor(&self.iv).encrypt(bytes, &self.key)
        }
    }

    pub fn encrypt_cbc(&self, bytes: &[u8]) -> Vec<u8> {
        let plain_bytes = self.get_plain_bytes(bytes);
        Aes128CbcEncryptor(&self.iv).encrypt(&plain_bytes, &self.key)
    }

    pub fn decrypt_cbc(&self, cipher_bytes: &[u8]) -> Vec<u8> {
        Aes128CbcDecryptor(&self.iv).decrypt(&cipher_bytes, &self.key)
    }
}
