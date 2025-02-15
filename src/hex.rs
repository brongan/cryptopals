use itertools::Itertools;

pub fn decode(input: &str) -> Vec<u8> {
    input
        .bytes()
        .map(|b| match b as char {
            '0'..='9' => b - b'0',
            'a'..='f' => b - b'a' + 10,
            _ => panic!("invalid character: {:?}", b as char),
        })
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap() << 4) | chunk.next().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn encode(input: &[u8]) -> String {
    let encode_quartet = |b| match b {
        0..=9 => (b'0' + b) as char,
        10..=15 => (b'a' + (b - 10)) as char,
        _ => panic!("invalid quartet: {b}"),
    };

    input
        .iter()
        .flat_map(|b| {
            let l = b >> 4;
            let r = (b << 4) >> 4;
            [encode_quartet(l), encode_quartet(r)]
        })
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
        let arr = b"Man";
        assert_eq!(decode(&encode(arr)), arr);
    }
}
