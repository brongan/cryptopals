use std::fmt::Display;

use cryptopals::hex;

fn byte_xor(input: &[u8], cipher: u8) -> Vec<u8> {
    input.into_iter().map(|b| b ^ cipher).collect()
}

#[derive(Debug)]
struct Decrypted {
    msg: String,
    cipher: u8,
}

impl Decrypted {
    fn score(&self) -> usize {
        self.msg.chars().filter(|c| c.is_ascii_graphic()).count()
            + 10 * self.msg.chars().filter(|c| *c == ' ').count()
    }
}

impl Display for Decrypted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "score: {}, cipher: {}, msg: {:?}",
            self.score(),
            self.cipher,
            self.msg
        )
    }
}

fn best_match(input: &[u8]) -> Decrypted {
    ascii_matches(input)
        .into_iter()
        .max_by(|l, r| l.score().cmp(&r.score()))
        .unwrap()
}

fn ascii_matches(input: &[u8]) -> Vec<Decrypted> {
    (0..255)
        .filter_map(|cipher| {
            let decrypted = byte_xor(&input, cipher);
            String::from_utf8(decrypted)
                .ok()
                .map(|msg| Decrypted { msg, cipher })
        })
        .filter(|Decrypted { msg, cipher: _ }| msg.chars().all(|c| c.is_ascii()))
        .collect()
}

fn challenge_3() {
    let input = String::from_utf8(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".into(),
    )
    .unwrap();
    let input = hex::decode(&input);
    let best = best_match(&input);

    println!("{best}");
}

fn challenge_4() {
    let input = include_str!("../../4.txt");
    let decrypted: Vec<_> = input
        .lines()
        .map(|line| {
            let line = hex::decode(line);
            ascii_matches(&line)
        })
        .flatten()
        .collect();

    let best = decrypted
        .into_iter()
        .max_by_key(|decrypted| decrypted.score())
        .unwrap();
    println!("{best}");
}

fn main() {
    challenge_3();
    challenge_4();
}
