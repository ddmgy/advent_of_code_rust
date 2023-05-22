const PADDING: [u8; 64] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const K: [u32; 64] = [
    0xD76AA478, 0xE8C7B756, 0x242070DB, 0xC1BDCEEE, 0xF57C0FAF, 0x4787C62A, 0xA8304613, 0xFD469501,
    0x698098D8, 0x8B44F7AF, 0xFFFF5BB1, 0x895CD7BE, 0x6B901122, 0xFD987193, 0xA679438E, 0x49B40821,
    0xF61E2562, 0xC040B340, 0x265E5A51, 0xE9B6C7AA, 0xD62F105D, 0x02441453, 0xD8A1E681, 0xE7D3FBC8,
    0x21E1CDE6, 0xC33707D6, 0xF4D50D87, 0x455A14ED, 0xA9E3E905, 0xFCEFA3F8, 0x676F02D9, 0x8D2A4C8A,
    0xFFFA3942, 0x8771F681, 0x6D9D6122, 0xFDE5380C, 0xA4BEEA44, 0x4BDECFA9, 0xF6BB4B60, 0xBEBFBC70,
    0x289B7EC6, 0xEAA127FA, 0xD4EF3085, 0x04881D05, 0xD9D4D039, 0xE6DB99E5, 0x1FA27CF8, 0xC4AC5665,
    0xF4292244, 0x432AFF97, 0xAB9423A7, 0xFC93A039, 0x655B59C3, 0x8F0CCC92, 0xFFEFF47D, 0x85845DD1,
    0x6FA87E4F, 0xFE2CE6E0, 0xA3014314, 0x4E0811A1, 0xF7537E82, 0xBD3AF235, 0x2AD7D2BB, 0xEB86D391,
];

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

#[derive(Clone, Eq, PartialEq)]
pub struct Digest([u8; 16]);

impl std::convert::From<Digest> for [u8; 16] {
    #[inline]
    fn from(value: Digest) -> Self {
        value.0
    }
}

impl std::ops::Deref for Digest {
    type Target = [u8; 16];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Digest {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(self, f)
    }
}

macro_rules! implement_hex_fmt {
    ($type:ident, $format:expr) => {
        impl std::fmt::$type for Digest {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for byte in &self.0 {
                    write!(f, $format, byte)?;
                }

                Ok(())
            }
        }
    };
}

implement_hex_fmt!(UpperHex, "{:02X}");
implement_hex_fmt!(LowerHex, "{:02x}");

macro_rules! make_chunk {
    ($buffer:expr) => ({
        let mut chunk = [0u32; 16];
        let mut j = 0;
        for i in 0..16 {
            chunk[i] = {
                (($buffer[j + 3] as u32) << 24) |
                (($buffer[j + 2] as u32) << 16) |
                (($buffer[j + 1] as u32) << 8) |
                $buffer[j] as u32
            };

            j += 4;
        }

        chunk
    });
}

fn transform(state: &mut [u32; 4], chunk: &[u32; 16]) {
    let (mut a, mut b, mut c, mut d) = (state[0], state[1], state[2], state[3]);

    macro_rules! add {
        ($a:expr, $b:expr) => ($a.wrapping_add($b));
    }

    for i in 0usize..64 {
        let (f, g) = match i {
            0..=15 => (
                (b & c) | ((!b) & d),
                i,
            ),
            16..=31 => (
                (d & b) | ((!d) & c),
                (5 * i + 1) % 16,
            ),
            32..=47 => (
                b ^ c ^ d,
                (3 * i + 5) % 16,
            ),
            48..=63 => (
                c ^ (b | (!d)),
                (7 * i) % 16,
            ),
            _ => unreachable!(),
        };

        let f = add!(add!(add!(f, a), K[i]), chunk[g]);
        a = d;
        d = c;
        c = b;
        b = add!(b, f.rotate_left(S[i]));    }


    state[0] = add!(state[0], a);
    state[1] = add!(state[1], b);
    state[2] = add!(state[2], c);
    state[3] = add!(state[3], d);
}

fn update<T>(
    State {
        state,
        length,
        buffer,
    }: &mut State,
    data: T,
) where T: AsRef<[u8]>
{
    let mut bytes_written = 0u64;
    let mut i = (*length % 64) as usize;

    for &byte in data.as_ref() {
        buffer[i] = byte;
        i += 1;

        if i == 0x40 {
            let chunk = make_chunk!(buffer);
            transform(state, &chunk);
            i = 0;
        }

        bytes_written = bytes_written.wrapping_add(1);
    }

    *length = length.wrapping_add(bytes_written);
}

#[derive(Clone)]
pub struct State {
    state: [u32; 4],
    length: u64,
    buffer: [u8; 64],
}

impl State {
    pub fn new() -> Self {
        Self {
            state: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476],
            length: 0,
            buffer: [0; 64],
        }
    }

    /// Add the bytes in [data] to the current [State].
    pub fn update<T>(&mut self, data: T)
    where T: AsRef<[u8]>
    {
        update(self, data)
    }

    /// Construct [Digest] from this [State].
    pub fn digest(self) -> Digest {
        let mut state = self;
        let original_length = state.length;
        let i = (state.length % 64) as usize;
        update(&mut state, &PADDING[..(if i < 56 { 56 - i } else { 120 - i })]);

        let mut length = original_length.saturating_mul(8);
        let mut length_buffer = [0u8; 8];
        for i in 0..8 {
            length_buffer[i] = (length & 0xFF) as u8;
            length >>= 8;
        }
        update(&mut state, length_buffer);

        let mut digest = [0u8; 16];
        let mut j = 0;
        for i in 0..4 {
            digest[j] = ((state.state[i]) & 0xFF) as u8;
            digest[j + 1] = ((state.state[i] >> 8) & 0xFF) as u8;
            digest[j + 2] = ((state.state[i] >> 16) & 0xFF) as u8;
            digest[j + 3] = ((state.state[i] >> 24) & 0xFF) as u8;
            j += 4;
        }

        Digest(digest)
    }
}

pub fn md5<T: AsRef<[u8]>>(data: T) -> Digest {
    let mut state = State::new();
    state.update(data);
    state.digest()
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_md5() {
        let inputs = [
            "",
            "a",
            "abc",
            "message digest",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
            "The quick brown fox jumps over the lazy dog",
            "The quick brown fox jumps over the lazy dog.",
        ];
        let outputs = [
            "d41d8cd98f00b204e9800998ecf8427e",
            "0cc175b9c0f1b6a831c399e269772661",
            "900150983cd24fb0d6963f7d28e17f72",
            "f96b697d7cb7938d525a2f31aaf161d0",
            "c3fcd3d76192e4007dfb496cca67e13b",
            "d174ab98d277d9f5a5611c2c9f419d9f",
            "57edf4a22be3c955ac49da2e2107b67a",
            "9e107d9d372bb6826bd81d3542a419d6",
            "e4d909c290d0fb1ca068ffaddf22cbd0",
        ];

        for (&input, &output) in inputs.iter().zip(outputs.iter()) {
            let mut state = super::State::new();
            state.update(input);
            let digest = state.digest();
            assert_eq!(output, format!("{:x}", digest));
        }
    }
}
