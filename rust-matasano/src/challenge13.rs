use libs::oracle::Oracle;

use std::str::from_utf8;
use regex::Regex;

pub fn run() {
    let problem = Problem::new();

    // Choose an email id such that the last word of the 
    // profile qurey string ('user') forms a brand new block
    // in the end
    let mut cipher_bytes = problem.encrypt_profile("zepr@kite.com");

    // Remove user
    cipher_bytes.truncate(32);

    // Add admin
    let cipher_admin = get_cipher_admin(&problem);
    cipher_bytes.extend(cipher_admin.iter());
    println!("Challenge 13 : {}", problem.decrypt_profile(&cipher_bytes));
}

fn get_cipher_admin(problem: &Problem) -> Vec<u8> {
    let mut  admin = String::from("admin");
    let pad_byte = 16 - admin.len() as u8;

    for _ in 0..pad_byte {
        admin.push(pad_byte as char);
    }

    let input = String::from("foobar@foo") + &admin;
    let cipher_bytes = problem.encrypt_profile(&input);

    cipher_bytes.iter()
        .skip(16)
        .take(16)
        .cloned()
        .collect()
}

struct Problem {
    oracle: Oracle
}

impl Problem {

    fn new() -> Self {
        Problem { oracle: Oracle::new(None, None) }
    }

    fn decode(text: &str) -> Result<String, &'static str> {

        if !Problem::is_query_string(text) {
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

    fn encrypt_profile(&self, email: &str) -> Vec<u8> {
        let profile = Problem::profile_for(email);
        self.oracle.encrypt(profile.as_bytes())
    }

    pub fn decrypt_profile(&self, cipher_bytes: &[u8]) -> String {
        let plain_bytes = self.oracle.decrypt(cipher_bytes);
        let plain_text = from_utf8(&plain_bytes).unwrap();
        Problem::decode(plain_text).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_query_string() {
        assert!(Problem::is_query_string("foo=bar&bar=baz&charlie=delta"));
        assert!(Problem::is_query_string("foo=bar@bar.baz&charlie=delta"));
    }

    #[test]
    fn test_is_query_string_negative() {
        assert!(!Problem::is_query_string("foo=bar&bar=baz&charlie="));
        assert!(!Problem::is_query_string("foo=bar&bar=baz&charlie"));
        assert!(!Problem::is_query_string("foo=bar&bar=baz&"));
        assert!(!Problem::is_query_string("foo=bar&=baz&charlie=delta"));
    }

    #[test]
    fn test_decode() {
        let result = Problem::decode("foo=bar&bar=baz&charlie=delta");
        assert!(result.is_ok());
        assert_eq!("{\n\tfoo : 'bar'\n\tbar : 'baz'\n\tcharlie : 'delta'\n}", result.unwrap());
    }

    #[test]
    fn test_decode_fail() {
        assert!(Problem::decode("foo=bar&bar=baz&charlie").is_err());
    }

    #[test]
    fn test_profile_for() {
        let profile = Problem::profile_for("foo@bar.baz");
        assert_eq!("email=foo@bar.baz&uid=10&role=user", profile);
    }

    #[test]
    fn test_profile_for_with_special_chars() {
        assert_eq!("email=foo@bar.bazroleadmin&uid=10&role=user",
                   Problem::profile_for("foo@bar.baz&role=admin"));
    }

    #[test]
    fn test_encrypt_decrypt() {
        let problem = Problem::new();
        let cipher_bytes = problem.encrypt_profile("foo@bar.baz");
        let profile = problem.decrypt_profile(&cipher_bytes);
        assert_eq!("{\n\temail : 'foo@bar.baz'\n\tuid : '10'\n\trole : 'user'\n}", profile);
    }
}
