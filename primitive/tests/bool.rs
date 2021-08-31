use primitive::decoder::decode;
use primitive::encoder::PrimitivePacketEncoder;
use primitive::PrimitivePacket;

#[test]
fn test_true() {
    let value = true;
    let mut enc = PrimitivePacketEncoder::new(0x2C);
    enc.from_bool(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x2C, 0x01, 0x01]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_bool() {
                Ok(retval) => assert_eq!(retval, value),
                Err(msg) => panic!("{}", msg),
            }
            assert_eq!(state.pos,  3);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}

#[test]
fn test_false() {
    let value = false;
    let mut enc = PrimitivePacketEncoder::new(0x2C);
    enc.from_bool(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x2C, 0x01, 0x00]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_bool() {
                Ok(retval) => assert_eq!(retval, value),
                Err(msg) => panic!("{}", msg),
            }
            assert_eq!(state.pos,  3);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}
