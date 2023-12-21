pub fn solve() -> i64 {
    let cipher_bytes = std::fs::read_to_string("res/xor_decryption.txt")
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>();
    for k0 in b'a'..=b'z' {
        for k1 in b'a'..=b'z' {
            for k2 in b'a'..=b'z' {
                let plain_bytes = [k0, k1, k2]
                    .iter()
                    .cycle()
                    .zip(cipher_bytes.iter())
                    .map(|(key_byte, cipher_byte)| key_byte ^ cipher_byte)
                    .collect::<Vec<u8>>();
                let plaintext = std::str::from_utf8(&plain_bytes).unwrap();
                if plaintext.contains(" the ")
                    && plaintext.contains(" it ")
                    && plaintext.contains(" to ")
                    && plaintext.contains(" from ")
                    && plaintext.contains(" of ")
                {
                    println!("{}", plaintext);
                }
            }
        }
    }

    0
}
