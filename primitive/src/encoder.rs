use codec::varint;
use codec::Codec;
use encoder::Encoder;

#[derive(Debug)]
pub struct PrimitivePacketEncoder {
    pub meta: Encoder,
}

impl PrimitivePacketEncoder {
    pub fn new(seq_id: u8) -> PrimitivePacketEncoder {
        PrimitivePacketEncoder {
            meta: Encoder {
                seq_id: seq_id,
                ..Default::default()
            },
        }
    }

    pub fn from_int32(&mut self, value: i32) {
        let size = varint::size_of_varint32(value);
        let mut codec = Codec { ptr: 0, size: size };
        self.meta.byte_array = vec![0; size];
        match codec.encode_varint32(&mut self.meta.byte_array, value) {
            Err(msg) => panic!("{}", msg),
            _ => (),
        }
    }

    pub fn from_uint32(&mut self, value: u32) {
        let size = varint::size_of_varuint32(value);
        let mut codec = Codec { ptr: 0, size: size };
        self.meta.byte_array = vec![0; size];
        match codec.encode_varuint32(&mut self.meta.byte_array, value) {
            Err(msg) => panic!("{}", msg),
            _ => (),
        }
    }

    // pub fn from_int64(&self, value: i64);
    // pub fn from_uint64(&self, value: u64);
    // pub fn from_float32(&self, value: f32);
    // pub fn from_float64(&self, value: f64);
    // pub fn from_utf8_string(&self, value: String);
    // pub fn from_bytes(&self, value: Vec<u8>);
}
