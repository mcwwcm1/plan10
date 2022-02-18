use crate::leb128::{encode_leb128, predict_size};
use bytes::{BufMut, Bytes};
use smallvec::SmallVec;
use std::ops::Range;
use thiserror::Error;

pub struct Packet {
    ack: Option<SmallVec<[Ack; 16]>>,
    data: Option<SmallVec<[Data; 16]>>,
}

pub enum Ack {
    Single(u16),
    Range(Range<u16>),
}

pub struct Data {
    channel: u8,
    data: Bytes,
}

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("The packet is empty, it carries no data and is therefore not serializable")]
    NothingToEncode,
    #[error("To many acks/data shards")]
    ArraysSpilled,
}

impl Packet {
    pub fn serialize(&self, mut buf: impl BufMut) -> Result<(), SerializationError> {
        let acks = self.ack.as_ref().map(|x| x.as_slice()).unwrap_or(&[]);
        let data = self.data.as_ref().map(|x| x.as_slice()).unwrap_or(&[]);

        if acks.len() + data.len() == 0 {
            return Err(SerializationError::NothingToEncode);
        } else if acks.len() > 16 || data.len() > 16 {
            return Err(SerializationError::ArraysSpilled);
        }

        let ack_size = acks
            .iter()
            .map(|ack| match ack {
                Ack::Single(_) => 2,
                Ack::Range(_) => 4,
            })
            .sum::<usize>();
        let data_size = data
            .iter()
            .map(|datum| 1 + predict_size(datum.data.len() as u64) + datum.data.len())
            .sum::<usize>();
        let size = 1 + ack_size + data_size;

        encode_leb128(size as u64, &mut buf);

        let comp_size = (acks.len() << 4 | data.len()) as u8;
        buf.put_u8(comp_size);

        for ack in acks {
            match ack {
                Ack::Single(x) => buf.put_u16_le(*x),
                Ack::Range(Range { start, end }) => {
                    buf.put_u16_le(*start);
                    buf.put_u16_le(*end);
                }
            }
        }

        for datum in data {
            buf.put_u8(datum.channel);
            encode_leb128(datum.data.len() as u64, &mut buf);
            buf.put_slice(&datum.data)
        }

        Ok(())
    }
}
