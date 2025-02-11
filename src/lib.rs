pub mod base64;
pub mod hex;

#[allow(dead_code)]
pub fn xor(l: &[u8], r: &[u8]) -> Vec<u8> {
    if l.len() != r.len() {
        panic!("cannot xor!");
    }
    l.iter().zip(r).map(|(l, r)| l ^ r).collect()
}

#[allow(dead_code)]
pub fn hex_to_base64(input: &str) -> String {
    let bytes = hex::decode(input);
    base64::encode(&bytes)
}

#[allow(dead_code)]
fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .map(|(i, b)| key[i % key.len()] ^ b)
        .collect()
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

    #[test]
    fn challenge_5() {
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        assert_eq!(
            hex::encode(&repeating_key_xor(input, key)),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }
}
