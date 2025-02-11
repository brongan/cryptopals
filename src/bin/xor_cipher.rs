use cryptopals::hex;

fn cipher_score(decoded: &str) -> usize {
    if !decoded.chars().map(|c| c.is_ascii()).all(|x| x) {
        return 0;
    }
    if decoded.chars().filter(|c| *c == '\n').count() > 0 {
        return 0;
    }
    decoded.chars().filter(|c| *c == ' ').count()
}

fn byte_decrypt(input: &[u8], cipher: u8) -> Vec<u8> {
    input.into_iter().map(|b| b ^ cipher).collect()
}

fn best_match(input: &[u8]) -> (String, u8) {
    (0..255)
        .filter_map(|cipher| {
            let decrypted = byte_decrypt(&input, cipher);
            String::from_utf8(decrypted).ok().map(|msg| (msg, cipher))
        })
        .max_by(|(l, c1), (r, c2)| cipher_score(l).cmp(&cipher_score(&r)))
        .unwrap()
}

fn main() {
    let input = String::from_utf8(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".into(),
    )
    .unwrap();
    let input = hex::decode(&input);
    let (best, cipher) = best_match(&input);

    println!("Encryption Key: \"{cipher}\" = \"{best}\"");

    let input = include_str!("../../4.txt");
    let lines: Vec<String> = input
        .lines()
        .map(|line| {
            let line = hex::decode(line);
            (0..255)
                .map(|cipher| byte_decrypt(&line, cipher))
                .filter_map(|decrypted| String::from_utf8(decrypted).ok())
                .collect::<Vec<_>>()
            // .max_by(|x, y| cipher_score(x).cmp(&cipher_score(y)))
        })
        .flatten()
        .collect();

    for (i, line) in lines.iter().enumerate() {
        println!("{i}: {line:?}");
    }
}
