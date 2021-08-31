use primitive::decoder::decode;
use primitive::PrimitivePacket;

#[test]
fn invalid_buffer_size() {
	let buffer = vec![0x01];
	let mut pp: PrimitivePacket = Default::default();
	assert!(decode(&buffer, &mut pp).is_err())
}

#[test]
fn zero_length() {
	let buffer = vec![0x04, 0x00, 0x03];
	let mut pp: PrimitivePacket = Default::default();
	match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(pp.meta.raw_bytes(), vec![0x04, 0x00]);
            assert_eq!(state.pos,  2);
            assert_eq!(state.size, 1);
			assert_eq!(pp.meta.byte_array.len(), 0);
        }
        Err(msg) => panic!("{}", msg),
    }

	let buffer = vec![0x04, 0x00];
	let mut pp: PrimitivePacket = Default::default();
	match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            assert_eq!(state.pos,  2);
            assert_eq!(state.size, 1);
			assert!(!pp.meta.is_slice());
        }
        Err(msg) => panic!("{}", msg),
    }
}

#[test]
fn negative_length() {
	let buffer = vec![0x04, 0x74, 0x01, 0x01];
	let mut pp: PrimitivePacket = Default::default();
	assert!(decode(&buffer, &mut pp).is_err());
	let raw_bytes =  pp.meta.raw_bytes();
	assert_eq!(raw_bytes.len(), 2);
	assert_eq!(raw_bytes[0], 0x04);
	assert_eq!(raw_bytes[1], 0x74);
	assert_eq!(pp.meta.byte_array.len(), 0);
}

#[test]
fn wrong_length() {
	let buffer = vec![0x0A, 0x70, 0x01, 0x02];
	let mut pp: PrimitivePacket = Default::default();
	assert!(decode(&buffer, &mut pp).is_err());
	let raw_bytes =  pp.meta.raw_bytes();
	assert_eq!(raw_bytes.len(), 2);
	assert_eq!(raw_bytes[0], 0x0A);
	assert_eq!(raw_bytes[1], 0x70);
	assert_eq!(pp.meta.byte_array.len(), 0);
}
