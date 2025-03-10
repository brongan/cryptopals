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
    score: i64,
}

impl Decrypted {
    fn new(msg: String, cipher: u8) -> Decrypted {
        let score = msg
            .chars()
            .map(|c| {
                match c {
                    '!'..='~' => 1, // is_ascii_graphic
                    ' ' => 10,
                    _ => 0,
                }
            })
            .sum();
        Decrypted { msg, cipher, score }
    }
}

impl Display for Decrypted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self { msg, cipher, score } = &self;
        write!(f, "score: {score}, cipher: {cipher:08b}, msg: {msg:?}",)
    }
}

fn single_byte_decrypt(input: &[u8]) -> Decrypted {
    single_byte_decrypt_candidates(input)
        .into_iter()
        .max_by(|l, r| l.score.cmp(&r.score))
        .unwrap()
}

fn single_byte_decrypt_candidates(input: &[u8]) -> Vec<Decrypted> {
    (0..=255)
        .filter_map(|cipher| {
            let decrypted = byte_xor(input, cipher);
            String::from_utf8(decrypted)
                .ok()
                .filter(|msg| msg.is_ascii())
                .map(|msg| Decrypted::new(msg, cipher))
        })
        .collect()
}

fn challenge_3() -> Decrypted {
    let input = String::from_utf8(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".into(),
    )
    .unwrap();
    let input = hex::decode(&input);
    single_byte_decrypt(&input)
}

fn challenge_4() -> Decrypted {
    include_str!("../../4.txt")
        .lines()
        .map(hex::decode)
        .flat_map(|line| single_byte_decrypt_candidates(&line))
        .max_by_key(|decrypted| decrypted.score)
        .unwrap()
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
        .combinations(2)
        .map(|combination| hamming_distance(combination[0], combination[1]) as f64 / keysize as f64)
        .sum::<f64>()
}

fn challenge_6() -> (String, String) {
    let input = base64::decode(&include_str!("../../6.txt").replace("\n", ""));

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
    // For each block, the single-byte XOR key that produces the best looking histogram
    // is the repeating-key XOR key byte for that block. Put them together and you have the key.
    let key: Vec<u8> = blocks
        .into_iter()
        .map(|block| single_byte_decrypt(&block).cipher)
        .collect();

    let decrypted = repeating_key_xor(&input, &key);
    (
        String::from_utf8(key).unwrap(),
        String::from_utf8(decrypted).unwrap(),
    )
}

fn challenge_7() -> String {
    print!("Part 7: ");
    let input = base64::decode(&include_str!("../../7.txt").replace("\n", ""));

    let key = b"YELLOW SUBMARINE";
    todo!();
}

fn main() {
    println!("Challenge 3: {}", challenge_3());
    println!("Challenge 4: {}", challenge_4());
    let (key, plaintext) = challenge_6();
    println!("Challenge 6: key=\"{}\"; plaintext=\n{}", key, plaintext);
    println!("Challenge 7: {}", challenge_7());
}
