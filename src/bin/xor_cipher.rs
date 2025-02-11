use std::io::Read;

use cryptopals::hex;

fn cipher_score(decoded: &str) -> usize {
    decoded
        .chars()
        .filter(|c| *c == 'e' || *c == 'E' || *c == ' ')
        .count()
}

fn byte_decrypt(input: &[u8], cipher: u8) -> Vec<u8> {
    input.into_iter().map(|b| b ^ cipher).collect()
}

fn main() {
    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut input).unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = hex::decode(&input.trim());
    let mut output: Vec<String> = (0..255)
        .map(|cipher| byte_decrypt(&input, cipher))
        .filter_map(|decrypted| String::from_utf8(decrypted).ok())
        .collect();
    output.sort_by(|x, y| cipher_score(x).cmp(&cipher_score(y)));

    println!("Best String: {}", output.last().unwrap().trim());
}
