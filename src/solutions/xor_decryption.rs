pub fn solve() -> i64 {
    let cipher_bytes = std::fs::read_to_string("res/solutions/xor_decryption.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>();
    let mut key_bytes = [b'a', b'a', b'a'];
    let sum = loop {
        let plain_bytes = cipher_bytes
            .iter()
            .zip(key_bytes.iter().cycle())
            .map(|(cipher_byte, key_byte)| cipher_byte ^ key_byte)
            .collect::<Vec<u8>>();
        let plaintext = std::str::from_utf8(&plain_bytes).unwrap();
        if plaintext.contains(" the ") {
            break plain_bytes.iter().fold(0i64, |sum, &byte| sum + byte as i64);
        }

        // Set up the next key.
        if key_bytes[2] != b'z' {
            key_bytes[2] += 1;
        } else {
            key_bytes[2] = b'a';
            if key_bytes[1] != b'z' {
                key_bytes[1] += 1;
            } else {
                key_bytes[1] = b'a';
                key_bytes[0] += 1;
            }
        }
    };

    assert_eq!(sum, 129448);
    sum
}
