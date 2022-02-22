///! How it works: https://en.wikipedia.org/wiki/LEB128

use bytes::{Buf, BufMut};
use nom::Needed;
use std::cmp::min;

pub fn encode_leb128(mut n: u64, mut buf: impl BufMut) {
    loop {
        let mut byte = (n & 0x7F) as u8;
        n >>= 7;

        if n != 0 {
            byte |= 0x80;
            buf.put_u8(byte);
        } else {
            buf.put_u8(byte);
            break;
        }
    }
}

pub fn predict_size(n: u64) -> usize {
    let set = (64 - n.leading_zeros()) as usize;
    set / 7 + (set % 7).min(1)
}

pub fn decode_leb128(mut buf: impl Buf) -> Result<u64, nom::Err<()>> {
    let mut n = 0;

    let mut shift = 0;

    while shift < 64 {
        if !buf.has_remaining() {
            return Err(nom::Err::Incomplete(Needed::Unknown));
        }
        let byte = buf.get_u8();
        n |= ((byte & 0x7F) as u64) << shift;

        if byte & 0x80 == 0 {
            return Ok(n);
        }
        shift += 7;
    }

    Err(nom::Err::Error(()))
}

#[cfg(test)]
mod tests {
    use crate::leb128::{decode_leb128, encode_leb128, predict_size};
    use bytes::BytesMut;

    #[test]
    fn encode_test() {
        let mut buffer = BytesMut::new();
        encode_leb128(0xFF, &mut buffer);
        assert_eq!(buffer.as_ref(), &[0xFF, 1]);
    }

    #[test]
    fn encode_test_len() {
        let mut buffer = BytesMut::new();
        encode_leb128(0xFFF_FFFF_FFFF_FFFF, &mut buffer);
        assert_eq!(buffer.len(), 9);
    }

    #[test]
    fn decode_test() {
        let decoded = decode_leb128(&[0xFF, 1][..]).unwrap();
        assert_eq!(decoded, 0xFF);
    }

    #[test]
    fn size_test() {
        assert_eq!(predict_size(0x7F), 1);
        assert_eq!(predict_size(0xFF), 2);
        assert_eq!(predict_size(0x7FFF_FFFF_FFFF_FFFF), 9);
        assert_eq!(predict_size(0xFFFF_FFFF_FFFF_FFFF), 10);
    }
}
