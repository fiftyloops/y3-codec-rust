use codec::Codec;
use packet::Packet;

pub mod encoder;
// pub mod decoder;

#[derive(Debug)]
pub struct PrimitivePacket {
    pub meta: Packet,
}

impl PrimitivePacket {
    pub fn to_int32(&self) -> Result<i32, String> {
        let mut codec = Codec {
            ptr: 0,
            size: self.meta.byte_array.len(),
        };
        codec.decode_varint32(&self.meta.byte_array)
    }

    pub fn to_uint32(&self) -> Result<u32, String> {
        let mut codec = Codec {
            ptr: 0,
            size: self.meta.byte_array.len(),
        };
        codec.decode_varuint32(&self.meta.byte_array)
    }

    // pub fn to_int64(&self) -> Result<i64, String>;
    // pub fn to_uint64(&self) -> Result<u64, String>;
    // pub fn to_float32(&self) -> Result<f32, String>;
    // pub fn to_float64(&self) -> Result<f64, String>;
    // pub fn to_utf8_string(&self) -> Result<String, String>;
    // pub fn to_bytes(&self) -> Result<Vec<u8>, String>;
}
