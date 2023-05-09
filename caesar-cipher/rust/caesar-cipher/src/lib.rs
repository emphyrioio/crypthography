mod cipher;

pub fn caesar_encrypt(i: u8, k: u8) -> u8 {
    (i + (k % 26)) % 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_encrypt() {
        assert_eq!(caesar_encrypt(0, 0), 0);
        assert_eq!(caesar_encrypt(0, 26), 0);
        assert_eq!(caesar_encrypt(0, 52), 0);
        assert_eq!(caesar_encrypt(0, 25), 25);
        assert_eq!(caesar_encrypt(10, 16), 0);
        assert_eq!(caesar_encrypt(25, 1), 0);
    }
}