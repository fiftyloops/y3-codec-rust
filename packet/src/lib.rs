use bytebuffer::ByteBuffer;
use tag::Tag;

#[derive(Debug)]
pub struct Packet {
    pub tag:        Tag,
    pub len:        usize,
    pub byte_array: Vec<u8>,
    pub buffer:     ByteBuffer,
}

impl Default for Packet {
    fn default() -> Packet {
        Packet {
            tag:        Default::default(),
            len:        Default::default(),
            byte_array: Default::default(),
            buffer:     ByteBuffer::new(),
        }
    }
}

impl Packet {
    #[allow(dead_code)]
    pub fn raw_bytes(&self) -> Vec<u8> {
        self.buffer.to_bytes()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[allow(dead_code)]
    pub fn seq_id(&self) -> u8 {
        self.tag.seq_id()
    }

    #[allow(dead_code)]
    pub fn is_slice(&self) -> bool {
        self.tag.is_slice()
    }

    #[allow(dead_code)]
    pub fn byte_array(&self) -> &Vec<u8> {
        &self.byte_array
    }
}
