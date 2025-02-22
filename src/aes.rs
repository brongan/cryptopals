// round keys are derived from the cipher key using the AES key schedule.
// AES requires a separate 128-bit round key block for each round plus one more.
fn key_expansion(state: &mut State, round_key: &[u8; 16]) {
    todo!();
}

// each byte of the state is combined with a byte of the round key using bitwise xor.
fn add_round_key(state: &mut State, round_key: &[u8; 16]) {
    todo!();
}

// a non-linear substitution step where each byte is replaced with another according to a lookup table.
fn sub_bytes(state: &mut State) {
    todo!();
}

// a transposition step where the last three rows of the state are shifted cyclically a certain number of steps.
fn shift_rows(state: &mut State) {
    todo!();
}

// a linear mixing operation which operates on the columns of the state, combining the four bytes in each column.
fn mix_columns(state: &mut State) {
    todo!();
}

type State = [u8; 16];

fn key_schedule(master_key: &[u8; 16], round: u8) -> [u8; 32] {
    let rc0 = match round {
        1 => 0x01,
        2 => 0x02,
        3 => 0x04,
        4 => 0x08,
        5 => 0x10,
        6 => 0x20,
        7 => 0x40,
        8 => 0x80,
        9 => 0x1B,
        10 => 0x36,
        _ => panic!("invalid round"),
    };
    let rc = [rc0, 0x00, 0x00, 0x00];
    todo!()
}

// Yi = F(PlainTexti, Key)
// Ciphertext = Yi
// 10 rounds for 128-bit keys.
// KeyExpansion
// AddRoundKey
// 9 rounds:
//	SubBytes
//	ShiftRows
//	MixColumns
//	AddRoundKey
// Final round:
//    SubBytes
//    ShiftRows
//    AddRoundKey
fn aes_128_ecb_decrypt(state: &mut State, master_key: &[u8; 16]) {
    key_expansion(state, key_schedule(master_key, 1));
    add_round_key(state, master_key);
    for _ in 1..=9 {
        sub_bytes(state);
        shift_rows(state);
        mix_columns(state);
        add_round_key(state, master_key);
    }
    sub_bytes(state);
    shift_rows(state);
    add_round_key(state, master_key);
}

pub fn aes_128_ecb_decrypt(ciphertext: &[u8], master_key: &[u8; 16]) -> Vec<u8> {
    if ciphertext.len() % 16 != 0 {
        panic!("aes_128_ecb requires ciphertext to be multiple of 16 bytes");
    }
    todo!();
}
