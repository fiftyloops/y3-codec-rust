use crate::PrimitivePacket;
use codec::Codec;
use packet::Packet;
use tag::Tag;

#[derive(Debug, Default)]
pub struct DecoderState {
    pub pos: usize,
    pub size: usize,
}

pub fn decode(buffer: &Vec<u8>, pp: &mut PrimitivePacket) -> Result<DecoderState, String> {
    if buffer.len() < 2 {
        return Err("invalid buffer size".to_string());
    }
    let mut pos = 0;
    pp.meta = Packet {
        tag: Tag { raw: buffer[pos] },
        ..Default::default()
    };
    pp.meta.buffer.write_u8(buffer[pos]);
    pos += 1;
    let mut state = DecoderState {
        pos: pos,
        ..Default::default()
    };
    let mut codec: Codec = Default::default();
    let len = match codec.decode_tagged_varint32(buffer[pos..].to_vec()) {
        Ok(value) => value as usize,
        Err(msg) => return Err(msg),
    };
    if codec.size < 1 {
        return Err("length must be at least 1 byte".to_string());
    }
    pp.meta.buffer.write_bytes(&buffer[pos..(pos + codec.size)]);
    pos += codec.size;

    state.pos = pos;
    state.size = codec.size;

    pp.meta.len = len;
    let end_pos = pos + len;
    if pos > end_pos || end_pos > buffer.len() || pos > buffer.len() {
        return Err("index out of bounds".to_string());
    }
    pp.meta.byte_array = buffer[pos..end_pos].to_vec();
    pp.meta.buffer.write_bytes(&buffer[pos..end_pos]);
    state.pos = end_pos;
    Ok(state)
}
