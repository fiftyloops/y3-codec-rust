use std::str;
use codec::Codec;
use packet::Packet;

pub mod decoder;
pub mod encoder;

#[derive(Debug)]
pub struct PrimitivePacket {
    pub meta: Packet,
}

impl Default for PrimitivePacket {
    fn default() -> PrimitivePacket {
        PrimitivePacket {
            meta: Default::default(),
        }
    }
}

impl PrimitivePacket {
    pub fn to_int32(&self) -> Result<i32, String> {
        let mut codec = Codec {
            ptr:    0,
            size:   self.meta.byte_array.len(),
        };
        codec.decode_varint32(&self.meta.byte_array)
    }

    pub fn to_uint32(&self) -> Result<u32, String> {
        let mut codec = Codec {
            ptr:    0,
            size:   self.meta.byte_array.len(),
        };
        codec.decode_varuint32(&self.meta.byte_array)
    }

    pub fn to_int64(&self) -> Result<i64, String> {
        let mut codec = Codec {
            ptr:    0,
            size:   self.meta.byte_array.len(),
        };
        codec.decode_varint64(&self.meta.byte_array)
    }

    pub fn to_uint64(&self) -> Result<u64, String> {
        let mut codec = Codec {
            ptr:    0,
            size:   self.meta.byte_array.len(),
        };
        codec.decode_varuint64(&self.meta.byte_array)
    }
    
    pub fn to_float32(&self) -> Result<f32, String> {
        let mut codec = Codec {
            ptr:    0,
            size:   self.meta.byte_array.len(),
        };
        codec.decode_varfloat32(&self.meta.byte_array)
    }

    pub fn to_float64(&self) -> Result<f64, String> {
        let mut codec = Codec {
            ptr:    0,
            size:   self.meta.byte_array.len(),
        };
        codec.decode_varfloat64(&self.meta.byte_array)
    }

    pub fn to_bool(&self) -> Result<bool, String> {
        let mut codec = Codec {
            ptr:    0,
            size:   self.meta.byte_array.len(),
        };
        codec.decode_tagged_varbool(&self.meta.byte_array)
    }

    pub fn to_utf8_string(&self) -> String {
        str::from_utf8(&self.meta.byte_array).unwrap().to_string()
    }
    
    pub fn to_bytes(&self) -> &Vec<u8> {
        &self.meta.byte_array
    }
}
