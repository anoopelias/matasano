use lib::oracle::Oracle;

pub fn run() {

    let problem = Problem::new();
    let value = String::from(":admin<true");
    let mut cipher_bytes = problem.encrypt_cookie(value);

    flip_last_bit(&mut cipher_bytes, 16);
    flip_last_bit(&mut cipher_bytes, 22);

    println!("Challenge 16 : isAdmin : {}", problem.is_admin(&cipher_bytes));
}


fn flip_last_bit(bytes: &mut Vec<u8>, pos: usize) {
    let byte = bytes.remove(pos);
    bytes.insert(pos, byte ^ 1);
}

struct Problem {
    oracle: Oracle
}

impl Problem {
    fn new() -> Self {
        let prepend = "comment1=cooking%20MCs;userdata=";
        let append = ";comment2=%20like%20a%20pound%20of%20bacon";
        let oracle = Oracle::new(Some(prepend.as_bytes().to_vec()),
            Some(append.as_bytes().to_vec()));

        Problem { oracle: oracle }
    }

    fn encrypt_cookie(&self, value: String) -> Vec<u8> {
        self.oracle.encrypt_cbc(value.replace(";", "\";\"")
           .replace("=", "\"=\"")
           .as_bytes())
    }

    fn is_admin(&self, cipher_bytes: &[u8]) -> bool {
        String::from_utf8_lossy(&self.oracle.decrypt_cbc(cipher_bytes))
            .contains(";admin=true;")
    }
}
