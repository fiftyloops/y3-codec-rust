use bytebuffer::ByteBuffer;

#[derive(Debug)]
pub struct Encoder {
    pub seq_id:     u8,
	pub byte_array: Vec<u8>,
	pub buffer:     ByteBuffer,
    pub is_node:    bool,
	pub is_array:   bool,
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
        if self.is_array {
            self.seq_id |= 0x40;
        }
        self.buffer.write_bytes(&vec![self.seq_id]);
    }

    // fn write_len(&mut self) {
    // }

    // fn encode(&self) -> {
    //     self.write_tag()
    //     self.write_len()
    //     self.buffer.write_bytes(enc.valbuf)
    // }
}
