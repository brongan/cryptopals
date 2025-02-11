use itertools::Itertools;

pub fn decode(input: &str) -> Vec<u8> {
    input
        .bytes()
        .into_iter()
        .map(|b| match b as char {
            '0'..='9' => b - '0' as u8,
            'a'..='f' => b - 'a' as u8 + 10,
            _ => panic!("invalid character: {:?}", b as char),
        })
        .chunks(2)
        .into_iter()
        .map(|mut chunk| chunk.next().unwrap() << 4 | chunk.next().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn encode(input: &[u8]) -> String {
    let encode_quartet = |b| match b {
        0..=9 => ('0' as u8 + b) as char,
        10..=15 => ('a' as u8 + (b - 10)) as char,
        _ => panic!("invalid quartet: {b}"),
    };

    input
        .into_iter()
        .map(|b| {
            let l = b >> 4;
            let r = (b << 4) >> 4;
            [encode_quartet(l), encode_quartet(r)]
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode(b"Ogres are like onions"),
            "4f6772657320617265206c696b65206f6e696f6e73"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("4f6772657320617265206c696b65206f6e696f6e73"),
            b"Ogres are like onions"
        );
    }

    #[test]
    fn test_hex() {
        let arr = &[b'M', b'a', b'n'];
        assert_eq!(decode(&encode(arr)), arr);
    }
}
