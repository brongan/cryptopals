#![feature(iter_map_windows)]
use std::fmt::Display;

use cryptopals::{base64, hamming_distance, hex, repeating_key_xor};
use itertools::Itertools;

fn byte_xor(input: &[u8], cipher: u8) -> Vec<u8> {
    input.iter().map(|b| b ^ cipher).collect()
}

#[derive(Debug)]
struct Decrypted {
    msg: String,
    cipher: u8,
}

impl Decrypted {
    fn score(&self) -> i64 {
        self.msg.chars().filter(|c| c.is_ascii_graphic()).count() as i64
            - self.msg.chars().count() as i64
            + 10 * self.msg.chars().filter(|c| *c == ' ').count() as i64
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
            let decrypted = byte_xor(input, cipher);
            String::from_utf8(decrypted)
                .ok()
                .map(|msg| Decrypted { msg, cipher })
        })
        .filter(|Decrypted { msg, cipher: _ }| msg.is_ascii())
        .collect()
}

fn challenge_3() -> Decrypted {
    let input = String::from_utf8(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".into(),
    )
    .unwrap();
    let input = hex::decode(&input);
    best_match(&input)
}

fn best_single_character_xor(input: &[Vec<u8>]) -> Decrypted {
    let decrypted: Vec<_> = input.iter().flat_map(|line| ascii_matches(line)).collect();

    decrypted
        .into_iter()
        .max_by_key(|decrypted| decrypted.score())
        .unwrap()
}

fn challenge_4() -> Decrypted {
    let input: Vec<_> = include_str!("../../4.txt")
        .lines()
        .map(hex::decode)
        .collect();
    best_single_character_xor(&input)
}

fn score_key_size(input: &[u8], keysize: usize) -> f64 {
    // For each KEYSIZE, take the first KEYSIZE worth of bytes,
    // and the second KEYSIZE worth of bytes,
    // and find the edit distance between them.
    // Normalize this result by dividing by KEYSIZE.
    // You could proceed perhaps with the smallest 2-3 KEYSIZE values.
    // Or take 4 KEYSIZE blocks instead of 2 and average the distances.
    let num_blocks = 4;
    input
        .chunks_exact(keysize)
        .take(num_blocks)
        .permutations(2)
        .map(|perm| hamming_distance(perm[0], perm[1]) as f64 / keysize as f64)
        .sum::<f64>()
}

fn challenge_6() -> String {
    let mut input = include_str!("../../6.txt").to_owned();
    input.retain(|c| !c.is_whitespace());
    let input = base64::decode(&input);

    // Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
    // The KEYSIZE with the smallest normalized edit distance is probably the key.
    let best_key_size = (2..=40)
        .min_by(|a, b| {
            score_key_size(&input, *a)
                .partial_cmp(&score_key_size(&input, *b))
                .unwrap()
        })
        .unwrap();

    // Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
    // Now transpose the blocks: make a block that is the first byte of every block,
    // and a block that is the second byte of every block, and so on.
    let mut blocks = vec![vec![]; best_key_size];
    for chunk in &input.iter().cloned().chunks(best_key_size) {
        for (i, byte) in chunk.into_iter().enumerate() {
            blocks[i].push(byte);
        }
    }

    // Solve each block as if it was single-character XOR. You already have code to do this.
    //  For each block, the single-byte XOR key that produces the best looking histogram
    //  is the repeating-key XOR key byte for that block. Put them together and you have the key.
    let key: Vec<u8> = blocks
        .into_iter()
        .map(|block| best_match(&block).cipher)
        .collect();

    let decrypted = repeating_key_xor(&input, &key);
    String::from_utf8(decrypted).unwrap()
}

fn challenge_7() -> String {
    let input = base64::decode(include_str!("../../7.txt"));
    let key = b"YELLOW SUBMARINE";
}

fn main() {
    println!("Challenge 3: {}", challenge_3());
    println!("Challenge 4: {}", challenge_4());
    println!("Challenge 6: {}", challenge_6());
    println!("Challenge 7: {}", challenge_7());
}
