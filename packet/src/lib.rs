use bytebuffer::ByteBuffer;

const SEQ_ID: u8 = 0x3F;
const NODE: u8 = 0x80;
const SLICE: u8 = 0x40;

#[derive(Debug)]
pub struct Tag {
    pub raw: u8,
}

#[derive(Debug)]
pub struct Packet {
    pub tag: Tag,
    pub len: usize,
	pub byte_array: Vec<u8>,
	pub buffer: ByteBuffer,
}

impl Tag{
    pub fn raw(&self) -> u8 {
        self.raw
    }

    pub fn is_node(&self) -> bool {
        self.raw & NODE == NODE
    }

    pub fn is_slice(&self) -> bool {
        self.raw & SLICE == SLICE
    }

    pub fn seq_id(&self) -> u8 {
        self.raw & SEQ_ID
    }
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
