#[cfg(test)]
use crate::cipher::{caesar_encrypt, caesar_decrypt};

#[test]
fn test_caesar_encrypt() {
    assert_eq!(caesar_encrypt(0, 0), 0);
    assert_eq!(caesar_encrypt(0, 26), 0);
    assert_eq!(caesar_encrypt(0, 52), 0);
    assert_eq!(caesar_encrypt(0, 25), 25);
    assert_eq!(caesar_encrypt(10, 16), 0);
    assert_eq!(caesar_encrypt(25, 1), 0);
}

#[test]
fn test_caesar_decrypt() {
    assert_eq!(caesar_decrypt(0, 0), 0);
    assert_eq!(caesar_decrypt(0, 26), 0);
    assert_eq!(caesar_decrypt(0, 52), 0);
    assert_eq!(caesar_decrypt(25, 25), 0);
    // assert_eq!(caesar_decrypt(0, 16), 10);
    // assert_eq!(caesar_decrypt(0, 1), 25);
}