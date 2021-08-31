use codec::Codec;
use codec::tagged_varint;
use codec::varint;
use codec::varfloat;
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

    pub fn from_int64(&mut self, value: i64) {
        let size = varint::size_of_varint64(value);
        let mut codec = Codec { ptr: 0, size: size };
        self.meta.byte_array = vec![0; size];
        match codec.encode_varint64(&mut self.meta.byte_array, value) {
            Err(msg) => panic!("{}", msg),
            _ => (),
        }
    }

    pub fn from_uint64(&mut self, value: u64) {
        let size = varint::size_of_varuint64(value);
        let mut codec = Codec { ptr: 0, size: size };
        self.meta.byte_array = vec![0; size];
        match codec.encode_varuint64(&mut self.meta.byte_array, value) {
            Err(msg) => panic!("{}", msg),
            _ => (),
        }
    }
    
    pub fn from_float32(&mut self, value: f32) {
        let size = varfloat::size_of_varfloat32(value);
        let mut codec = Codec { ptr: 0, size: size };
        self.meta.byte_array = vec![0; size];
        match codec.encode_varfloat32(&mut self.meta.byte_array, value) {
            Err(msg) => panic!("{}", msg),
            _ => (),
        }
    }

    pub fn from_float64(&mut self, value: f64) {
        let size = varfloat::size_of_varfloat64(value);
        let mut codec = Codec { ptr: 0, size: size };
        self.meta.byte_array = vec![0; size];
        match codec.encode_varfloat64(&mut self.meta.byte_array, value) {
            Err(msg) => panic!("{}", msg),
            _ => (),
        }
    }

    pub fn from_bool(&mut self, value: bool) {
        let size = tagged_varint::size_of_tagged_varuint32(1 as u32);
        let mut codec = Codec { ptr: 0, size: size };
        self.meta.byte_array = vec![0; size];
        match codec.encode_tagged_varbool(&mut self.meta.byte_array, value) {
            Err(msg) => panic!("{}", msg),
            _ => (),
        }
    }
    
    pub fn from_utf8_string(&mut self, value: String) {
        self.meta.byte_array = value.into_bytes();
    }

    pub fn from_bytes(&mut self, value: Vec<u8>) {
        self.meta.byte_array = value;
    }
}
