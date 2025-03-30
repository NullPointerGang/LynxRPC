use bytes::{BytesMut, BufMut, Buf};
use tokio_util::codec::{Encoder, Decoder};
use rmp_serde::{Serializer, Deserializer};
use serde::{Serialize, de::DeserializeOwned};
use std::io;

pub struct RpcCodec;

impl Encoder<(u32, Vec<u8>)> for RpcCodec {
    type Error = io::Error;

    fn encode(&mut self, (id, data): (u32, Vec<u8>), dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = data.len() as u32;
        dst.reserve(8 + data.len());
        dst.put_u32(id);
        dst.put_u32(len);
        dst.put_slice(&data);
        Ok(())
    }
}

impl Decoder for RpcCodec {
    type Item = (u32, BytesMut);
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 8 {
            return Ok(None);
        }

        let id = u32::from_be_bytes([src[0], src[1], src[2], src[3]]);
        let len = u32::from_be_bytes([src[4], src[5], src[6], src[7]]) as usize;

        if src.len() >= 8 + len {
            src.advance(8);
            let data = src.split_to(len);
            Ok(Some((id, data)))
        } else {
            Ok(None)
        }
    }
}

pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    let mut buf = Vec::new();
    value.serialize(&mut Serializer::new(&mut buf))?;
    Ok(buf)
}

pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, rmp_serde::decode::Error> {
    let mut de = Deserializer::new(bytes);
    T::deserialize(&mut de)
}