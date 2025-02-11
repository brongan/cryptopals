mod base64;
mod hex;

#[allow(dead_code)]
fn xor(l: &[u8], r: &[u8]) -> Vec<u8> {
    if l.len() != r.len() {
        panic!("cannot xor!");
    }
    l.into_iter().zip(r).map(|(l, r)| l ^ r).collect()
}

#[allow(dead_code)]
fn hex_to_base64(input: &str) -> String {
    let bytes = hex::decode(input);
    base64::encode(&bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xor() {
        let l = hex::decode("1c0111001f010100061a024b53535009181c");
        let r = hex::decode("686974207468652062756c6c277320657965");
        assert_eq!(
            hex::encode(&xor(&l, &r)),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    fn test_hex_to_base64() {
        const HEX: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        const BASE64: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(HEX), BASE64);
    }
}
