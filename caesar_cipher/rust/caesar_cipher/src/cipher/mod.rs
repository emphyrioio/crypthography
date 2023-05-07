pub fn caesar_encrypt(i: u8, k: u8) -> u8 {
    (i + (k % 26)) % 26
}

pub fn caesar_decrypt(i: u8, k: u8) -> u8 {
    (i - (k % 26)) % 26
}
