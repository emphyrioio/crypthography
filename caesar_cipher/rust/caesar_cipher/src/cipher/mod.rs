pub fn caesar_encrypt(i: u32, k: u32, m: u32) -> Option<u32> {
    if i > m {
        return None;
    }
    if m == 0 {
        return None;
    }
    let k = k.checked_rem(m)?;
    Some(i.checked_add(k)?.checked_rem(m)?)
}

pub fn caesar_decrypt(i: u32, k: u32, m: u32) -> Option<u32> {
    if i > m {
        return None;
    }
    if m == 0 {
        return None;
    }
    let k_inv = m.checked_sub(k)?.checked_rem(m)?;
    caesar_encrypt(i, k_inv, m)
}
