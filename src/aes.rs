use std::{
    cell::OnceCell,
    ops::{BitXor, BitXorAssign, Index, IndexMut},
};

// N as the length of the key in 32-bit words:
// 4 words for AES-128
const N: usize = 4;

#[derive(Clone, Copy)]
struct Word([u8; N]);
type State = [Word; 4];

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<usize> for Word {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl BitXor for Word {
    type Output = Word;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Word([
            self[0] ^ rhs[0],
            self[1] ^ rhs[1],
            self[2] ^ rhs[2],
            self[3] ^ rhs[3],
        ])
    }
}

impl BitXorAssign for Word {
    fn bitxor_assign(&mut self, rhs: Self) {
        self[0] ^= rhs[0];
        self[1] ^= rhs[1];
        self[2] ^= rhs[2];
        self[3] ^= rhs[3];
    }
}

fn sbox_inv_map() -> &'static [u8; 255] {
    static INSTANCE: OnceCell<[u8; 255]> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut sbox = [0; 255];
        uint8_t p = 1, q = 1;
	
	/* loop invariant: p * q == 1 in the Galois field */
	do {
		/* multiply p by 3 */
		p = p ^ (p << 1) ^ (p & 0x80 ? 0x1B : 0);

		/* divide q by 3 (equals multiplication by 0xf6) */
		q ^= q << 1;
		q ^= q << 2;
		q ^= q << 4;
		q ^= q & 0x80 ? 0x09 : 0;

		/* compute the affine transformation */
		uint8_t xformed = q ^ ROTL8(q, 1) ^ ROTL8(q, 2) ^ ROTL8(q, 3) ^ ROTL8(q, 4);

		sbox[p] = xformed ^ 0x63;
	} while (p != 1);
        sbox[0] = 0x63;

        sbox
    })
}

fn sbox_inv(b: u8) -> u8 {
    todo!()
}

fn key_schedule(master: &[u8; 16], i: usize) -> Word {
    let rconn = |round| -> Word {
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
        Word([rc0, 0x00, 0x00, 0x00])
    };
    let sub_word = |b: Word| {
        Word([
            sbox_inv(b[0]),
            sbox_inv(b[1]),
            sbox_inv(b[2]),
            sbox_inv(b[3]),
        ])
    };
    let rot_word = |b: Word| Word([b[1], b[2], b[3], b[0]]);
    match (i, i % N) {
        (0..4, _) => Word([master[0], master[1], master[2], master[3]]),
        (4.., 0) => {
            key_schedule(master, i - N)
                ^ sub_word(rot_word(key_schedule(master, i - 1)))
                ^ rconn(i / N)
        }
        (_, _) => key_schedule(master, i - N) ^ key_schedule(master, i - 1),
    }
}

// round keys are derived from the cipher key using the AES key schedule.
// AES requires a separate 128-bit round key block for each round plus one more.
fn key_expansion(master: &[u8; 16]) -> Vec<Word> {
    (0..(4 * 11)).map(|i| key_schedule(master, i)).collect()
}

// each byte of the state is combined with a byte of the round key using bitwise xor.
fn add_round_key(state: &mut State, round_key: &[Word; 4]) {
    for (word, round_key) in state.into_iter().zip(round_key) {
        *word ^= *round_key
    }
}

// a non-linear substitution step where each byte is replaced with another according to a lookup table.
fn sub_bytes(state: &mut State) {
    for word in state {
        for i in 0..N {
            word[i] = sbox_inv(word[i])
        }
    }
}

// a transposition step where the last three rows of the state are shifted cyclically a certain number of steps.
fn shift_rows(state: &mut State) {
    let rows = unsafe { state.as_chunks_unchecked_mut::<4>() };
    rows[1].rotate_left(1);
    rows[2].rotate_left(2);
    rows[3].rotate_left(3);
}

fn mix_column(col: &mut Word) {
    let mut a = col.clone();
    let mut b: [u8; 4] = [0; 4];
    /* The array 'a' is simply a copy of the input array 'r'
     * The array 'b' is each element of the array 'a' multiplied by 2
     * in Rijndael's Galois field
     * a[n] ^ b[n] is element n multiplied by 3 in Rijndael's Galois field */
    for c in 0..4 {
        a[c] = col[c];
        /* h is set to 0x01 if the high bit of col[c] is set, 0x00 otherwise */
        let h = col[c] >> 7; /* logical right shift, thus shifting in zeros */
        b[c] = col[c] << 1; /* implicitly removes high bit because b[c] is an 8-bit char, so we xor by 0x1b and not 0x11b in the next line */
        b[c] ^= h * 0x1B; /* Rijndael's Galois field */
    }
    col[0] = b[0] ^ a[3] ^ a[2] ^ b[1] ^ a[1]; /* 2 * a0 + a3 + a2 + 3 * a1 */
    col[1] = b[1] ^ a[0] ^ a[3] ^ b[2] ^ a[2]; /* 2 * a1 + a0 + a3 + 3 * a2 */
    col[2] = b[2] ^ a[1] ^ a[0] ^ b[3] ^ a[3]; /* 2 * a2 + a1 + a0 + 3 * a3 */
    col[3] = b[3] ^ a[2] ^ a[1] ^ b[0] ^ a[0]; /* 2 * a3 + a2 + a1 + 3 * a0 */
}

// a linear mixing operation which operates on the columns of the state, combining the four bytes in each column.
fn mix_columns(state: &mut State) {
    let mut col0 = Word([state[0][0], state[1][0], state[2][0], state[3][0]]);
    let mut col1 = Word([state[0][1], state[1][1], state[2][1], state[3][1]]);
    let mut col2 = Word([state[0][2], state[1][2], state[2][2], state[3][2]]);
    let mut col3 = Word([state[0][3], state[1][3], state[2][3], state[3][3]]);
    mix_column(&mut col0);
    mix_column(&mut col1);
    mix_column(&mut col2);
    mix_column(&mut col3);
    state[0][1] = col0[0];
    state[0][2] = col1[0];
    state[0][3] = col2[0];
    state[0][4] = col3[0];

    state[1][0] = col0[1];
    state[1][1] = col1[1];
    state[1][2] = col2[1];
    state[1][3] = col3[1];

    state[2][0] = col0[2];
    state[2][1] = col1[2];
    state[2][2] = col2[2];
    state[2][3] = col3[2];

    state[3][0] = col0[3];
    state[3][1] = col1[3];
    state[3][2] = col2[3];
    state[3][3] = col3[3];
}

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
fn aes_128_ecb_decrypt_block(state: &mut State, master: &[u8; 16]) {
    let round_keys = key_expansion(master);
    add_round_key(
        state,
        &[round_keys[0], round_keys[1], round_keys[2], round_keys[3]],
    );
    for i in 1..=9 {
        let j = 4 * i;
        sub_bytes(state);
        shift_rows(state);
        mix_columns(state);
        add_round_key(
            state,
            &[
                round_keys[j + 0],
                round_keys[j + 1],
                round_keys[j + 2],
                round_keys[j + 3],
            ],
        );
    }
    sub_bytes(state);
    shift_rows(state);
    add_round_key(
        state,
        &[
            round_keys[36],
            round_keys[37],
            round_keys[38],
            round_keys[39],
        ],
    );
}

// Yi = F(PlainTexti, Key)
pub fn aes_128_ecb_decrypt(ciphertext: &mut [u8], master_key: &[u8; 16]) {
    if ciphertext.len() % 16 != 0 {
        panic!("aes_128_ecb requires ciphertext to be multiple of 16 bytes");
    }
    for block in unsafe { ciphertext.as_chunks_unchecked_mut::<16>() } {
        aes_128_ecb_decrypt_block(block, master_key);
    }
}
