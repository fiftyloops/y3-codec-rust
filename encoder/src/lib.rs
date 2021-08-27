use bytebuffer::ByteBuffer;
use codec::Codec;
use codec::tagged_varlen_int;

#[derive(Debug)]
pub struct Encoder {
    pub seq_id:     u8,
	pub byte_array: Vec<u8>,
	pub buffer:     ByteBuffer,
    pub is_node:    bool,
	pub is_slice:   bool,
    pub complete:   bool,
}

impl Encoder {
    pub fn is_empty(&self) -> bool {
        self.byte_array.is_empty()
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.byte_array
    }

    pub fn add_bytes(&mut self, bytes: &mut Vec<u8>) {
        self.byte_array.append(bytes);
    }

    pub fn write_tag(&mut self) {
        if self.seq_id > 0x3F {
            panic!("seq_id must be between 0 and 0x3F (inclusive)");
        }
        if self.is_node {
            self.seq_id |= 0x80;
        }
        if self.is_slice {
            self.seq_id |= 0x40;
        }
        self.buffer.write_bytes(&vec![self.seq_id]);
    }

    pub fn write_len(&mut self) {
        let value = self.byte_array.len() as u32;
        let size = tagged_varlen_int::size_of_tagged_varlen_uint32(value);    
        let mut byte_array = vec![0; size];
        let mut codec = Codec {
            ptr:    0,
            size:   size,
        };
        match codec.encode_tagged_varlen_uint32(&mut byte_array, value) {
            Ok(_) => {
                self.buffer.write_bytes(&byte_array)
            },
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn encode(&mut self) -> Vec<u8> {
        if !self.complete {
            self.write_tag();
            self.write_len();
            self.buffer.write_bytes(&self.byte_array);
            self.complete = true;
        }
        self.buffer.to_bytes()
    }
}
