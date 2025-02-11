use itertools::Itertools;

fn encode_sextet(sextet: u8) -> char {
    match sextet {
        0..=25 => (b'A' + sextet) as char,
        26..=51 => (b'a' + sextet - 26) as char,
        52..=61 => (b'0' + (sextet - 52)) as char,
        62 => '+',
        63 => '/',
        _ => panic!("invalid sextet: {sextet}"),
    }
}

pub fn encode(bytes: &[u8]) -> String {
    // [0,0,0,0,0,0,1,1],[1,1,1,1,2,2,2,2],[2,2,3,3,3,3,3,3] Repeat
    let mut ret = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk[1];
        let b2 = chunk[2];

        let c0 = b0 >> 2;
        let c1 = ((b0 << 6) >> 2) | (b1 >> 4);
        let c2 = ((b1 << 4) >> 2) | (b2 >> 6);
        let c3 = (b2 << 2) >> 2;
        ret.push(encode_sextet(c0));
        ret.push(encode_sextet(c1));
        ret.push(encode_sextet(c2));
        ret.push(encode_sextet(c3));
    }
    ret
}

fn decode_sextet(c: char) -> u8 {
    match c {
        'A'..='Z' => (c as u8) - b'A',
        'a'..='z' => 26 + c as u8 - b'a',
        '0'..='9' => 52 + c as u8 - b'0',
        '+' => 62,
        '/' => 63,
        _ => panic!("invalid base64 char: {c}"),
    }
}

#[allow(dead_code)]
pub fn decode(input: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    for mut chunk in &input.chars().chunks(4) {
        let c0 = decode_sextet(chunk.next().unwrap());
        let c1 = decode_sextet(chunk.next().unwrap());
        let c2 = decode_sextet(chunk.next().unwrap());
        let c3 = decode_sextet(chunk.next().unwrap());

        let b0 = c0 << 2 | c1 >> 4;
        let b1 = c1 << 4 | c2 >> 2;
        let b2 = c2 << 6 | c3;
        ret.push(b0);
        ret.push(b1);
        ret.push(b2);
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode(b"Ogres are like onions"),
            "T2dyZXMgYXJlIGxpa2Ugb25pb25z"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("T2dyZXMgYXJlIGxpa2Ugb25pb25z"),
            b"Ogres are like onions"
        );
    }

    #[test]
    fn test_man() {
        assert_eq!(encode(b"Man"), "TWFu");
    }
}
