pub mod tag;
use bytebuffer::ByteBuffer;
use tag::Tag;

#[derive(Debug)]
pub struct Packet {
    pub tag: Tag,
    pub len: usize,
	pub byte_array: Vec<u8>,
	pub buffer: ByteBuffer,
}

impl Packet {
    fn raw_bytes(&self) -> Vec<u8> {
        self.buffer.to_bytes()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn seq_id(&self) -> u8 {
        self.tag.seq_id()
    }

    fn is_slice(&self) -> bool {
        self.tag.is_slice()
    }

    fn byte_array(&self) -> &Vec<u8> {
        &self.byte_array
    }
}
