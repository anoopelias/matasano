use lib::random::Random;
use lib::cryptor::Encryptor;
use lib::cryptor::Decryptor;
use lib::cryptor::Aes128EcbEncryptor;
use lib::cryptor::Aes128EcbDecryptor;
use lib::cryptor::Aes128CbcEncryptor;

use std::str::from_utf8;
use regex::Regex;

pub struct Oracle {
    random: Random,
    key: Vec<u8>,
    iv: Vec<u8>,
    unknown_bytes: Option<Vec<u8>>,
    prefix: Option<Vec<u8>>,
}

impl Oracle {

    pub fn new() -> Self {
        Oracle::new_oracle(None, false)
    }

    pub fn new_with_unknown_bytes(unknown_bytes: Vec<u8>) -> Self {
        Oracle::new_oracle(Some(unknown_bytes), false)
    }

    pub fn new_with_random_prefix(unknown_bytes: Vec<u8>) -> Self {
        Oracle::new_oracle(Some(unknown_bytes), true)
    }

    fn new_oracle(unknown_bytes: Option<Vec<u8>>, should_prefix: bool) -> Self {
        let mut random = Random::new();
        let key = &mut [0;16];
        let iv = &mut [0;16];
        let prefix;

        if should_prefix {
            prefix = Some(random.rand_bytes(&256));
        } else {
            prefix = None;
        }

        random.fill_bytes(key);
        random.fill_bytes(iv);

        Oracle { random: random, key: key.to_vec(), iv: iv.to_vec(),
            unknown_bytes: unknown_bytes, prefix: prefix}
    }

    pub fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let blank_slice = Vec::new();
        let mut plain_bytes = Vec::new();

        let prefix = self.prefix.as_ref().unwrap_or(&blank_slice);
        let suffix = self.unknown_bytes.as_ref().unwrap_or(&blank_slice);


        plain_bytes.extend(prefix);
        plain_bytes.extend(bytes);
        plain_bytes.extend(suffix);

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

    pub fn encrypt_profile(&self, email: &str) -> Vec<u8> {
        let profile = profile_for(email);
        self.encrypt(profile.as_bytes())
    }

    pub fn decrypt_profile(&self, cipher_bytes: &[u8]) -> String {
        let plain_bytes = self.decrypt(cipher_bytes);
        let plain_text = from_utf8(&plain_bytes).unwrap();
        decode(plain_text).unwrap()
    }

}

fn decode(text: &str) -> Result<String, &'static str> {

    if !is_query_string(text) {
        Err("Not a valid query string")
    } else {
        let re = Regex::new(r"([^&=]+)=([^&=]+)").unwrap();
        let mut json = String::new();
        json.push_str("{\n");
        for cap in re.captures_iter(text) {
            let attr = String::from("\t") + &cap[1] + " : '" + &cap[2] + "'\n";
            json.push_str(attr.as_str());
        }
        json.push_str("}");
        Ok(json)
    }
}

fn is_query_string(text: &str) -> bool {
    let re = Regex::new(r"^([^&=]+=[^&=]+)(&[^&=]+=[^&=]+)*$").unwrap();
    re.is_match(text)
}

fn profile_for(email: &str) -> String {
    let clean_email = &email.replace("&", "").replace("=", "");
    String::from("email=") + clean_email + "&uid=10&role=user"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_query_string() {
        assert!(is_query_string("foo=bar&bar=baz&charlie=delta"));
        assert!(is_query_string("foo=bar@bar.baz&charlie=delta"));
    }

    #[test]
    fn test_is_query_string_negative() {
        assert!(!is_query_string("foo=bar&bar=baz&charlie="));
        assert!(!is_query_string("foo=bar&bar=baz&charlie"));
        assert!(!is_query_string("foo=bar&bar=baz&"));
        assert!(!is_query_string("foo=bar&=baz&charlie=delta"));
    }

    #[test]
    fn test_decode() {
        let result = decode("foo=bar&bar=baz&charlie=delta");
        assert!(result.is_ok());
        assert_eq!("{\n\tfoo : 'bar'\n\tbar : 'baz'\n\tcharlie : 'delta'\n}", result.unwrap());
    }

    #[test]
    fn test_decode_fail() {
        assert!(decode("foo=bar&bar=baz&charlie").is_err());
    }

    #[test]
    fn test_profile_for() {
        let profile = profile_for("foo@bar.baz");
        assert_eq!("email=foo@bar.baz&uid=10&role=user", profile);
    }

    #[test]
    fn test_profile_for_with_special_chars() {
        assert_eq!("email=foo@bar.bazroleadmin&uid=10&role=user",
                   profile_for("foo@bar.baz&role=admin"));
    }

    #[test]
    fn test_encrypt_decrypt() {
        let oracle = Oracle::new();
        let cipher_bytes = oracle.encrypt_profile("foo@bar.baz");
        let profile = oracle.decrypt_profile(&cipher_bytes);
        assert_eq!("{\n\temail : 'foo@bar.baz'\n\tuid : '10'\n\trole : 'user'\n}", profile);
    }
}
