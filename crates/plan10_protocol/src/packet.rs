use crate::leb128::{encode_leb128, predict_size};
use crate::ChannelId;
use bytes::{BufMut, Bytes};
use smallvec::SmallVec;
use thiserror::Error;

pub struct Packet {
    data: Option<SmallVec<[Data; 16]>>,
}

pub struct Data {
    channel: ChannelId,
    data: Bytes,
}

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("The packet is empty, it carries no data and is therefore not serializable")]
    NothingToEncode,
    #[error("To many data fragments")]
    ArraySpilled,
}

impl Packet {
    pub fn serialize(&self, mut buf: impl BufMut) -> Result<(), SerializationError> {
        let data = self.data.as_ref().map(|x| x.as_slice()).unwrap_or(&[]);

        if data.len() == 0 {
            return Err(SerializationError::NothingToEncode);
        } else if data.len() > 16 {
            return Err(SerializationError::ArraySpilled);
        }

        // First we compute the size of our packet

        let data_size = data
            .iter()
            .map(|datum| 1 + predict_size(datum.data.len() as u64) + datum.data.len())
            .sum::<usize>();

        encode_leb128(data_size as u64, &mut buf);

        for datum in data {
            buf.put_u8(datum.channel);
            encode_leb128(datum.data.len() as u64, &mut buf);
            buf.put_slice(&datum.data)
        }

        Ok(())
    }
}
