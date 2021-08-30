const SEQ_ID: u8 = 0x3F;
const NODE: u8 = 0x80;
const SLICE: u8 = 0x40;

#[derive(Debug, Default)]
pub struct Tag {
    pub raw: u8,
}

impl Tag {
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
