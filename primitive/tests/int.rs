use primitive::decoder::decode;
use primitive::encoder::PrimitivePacketEncoder;
use primitive::PrimitivePacket;

#[test]
fn int32() {
    let value: i32 = 255;
    let mut enc = PrimitivePacketEncoder::new(0x0A);
    enc.from_int32(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0A, 0x02, 0x00, 0xFF]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_int32() {
                Ok(retval) => assert_eq!(retval, value),
                Err(msg) => panic!("{}", msg),
            }
            assert_eq!(state.pos,  4);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}

#[test]
fn uint32() {
    let value: u32 = 255;
    let mut enc = PrimitivePacketEncoder::new(0x0A);
    enc.from_uint32(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0A, 0x02, 0x00, 0xFF]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_uint32() {
                Ok(retval) => assert_eq!(retval, value),
                Err(msg) => panic!("{}", msg),
            }
            assert_eq!(state.pos,  4);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}

#[test]
fn int64() {
    let value: i64 = 255;
    let mut enc = PrimitivePacketEncoder::new(0x0A);
    enc.from_int64(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0A, 0x02, 0x00, 0xFF]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_int64() {
                Ok(retval) => assert_eq!(retval, value),
                Err(msg) => panic!("{}", msg),
            }
            assert_eq!(state.pos,  4);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}

#[test]
fn uint64() {
    let value: u64 = 255;
    let mut enc = PrimitivePacketEncoder::new(0x0A);
    enc.from_uint64(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0A, 0x02, 0x00, 0xFF]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_uint64() {
                Ok(retval) => assert_eq!(retval, value),
                Err(msg) => panic!("{}", msg),
            }
            assert_eq!(state.pos,  4);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}
