use lib::oracle::Oracle;

pub fn run() {
    let oracle = Oracle::new(None);
    let cipher_admin = oracle.encrypt("admin".as_bytes());

    // Choose an email id such that the last word of the 
    // profile qurey string ('user') forms a brand new block
    // in the end
    let mut cipher_bytes = oracle.encrypt_profile("zepr@kite.com");

    // Remove user
    cipher_bytes.truncate(32);

    // Add admin
    cipher_bytes.extend(cipher_admin.iter().cloned());
    println!("Challenge 13 : {}", oracle.decrypt_profile(&cipher_bytes));
}

