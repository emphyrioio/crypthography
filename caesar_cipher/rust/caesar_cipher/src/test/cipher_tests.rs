#[cfg(test)]
use crate::cipher::{caesar_decrypt, caesar_encrypt};

#[test]
fn test_caesar_encrypt() {
    assert_eq!(caesar_encrypt(0, 0, 26), Some(0));
    assert_eq!(caesar_encrypt(0, 26, 26), Some(0));
    assert_eq!(caesar_encrypt(0, 52, 26), Some(0));
    assert_eq!(caesar_encrypt(0, 25, 26), Some(25));
    assert_eq!(caesar_encrypt(10, 16, 26), Some(0));
    assert_eq!(caesar_encrypt(25, 1, 26), Some(0));
}

#[test]
fn test_caesar_decrypt() {
    assert_eq!(caesar_decrypt(0, 0, 26), Some(0));
    assert_eq!(caesar_decrypt(0, 26, 26), Some(0));
    assert_eq!(caesar_decrypt(25, 25, 26), Some(0));
}
