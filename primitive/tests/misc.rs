use primitive::decoder::decode;
use primitive::PrimitivePacket;

#[test]
fn basic() {
	let buffer = vec![0x04, 0x01, 0x7F];
	let mut pp: PrimitivePacket = Default::default();
	match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            assert_eq!(pp.meta.tag.seq_id(), 0x04);
            assert_eq!(pp.meta.len, 1);
            assert_eq!(pp.meta.byte_array.len(), 1);
            assert_eq!(pp.meta.byte_array[0], 0x7F);
            assert_eq!(state.pos,  3);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}
