trait Packet {
    fn raw_bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
    fn seq_id(&self) -> u8;
    fn is_slice(&self) -> bool;
    fn byte_array(&self) -> &Vec<u8>;
}
