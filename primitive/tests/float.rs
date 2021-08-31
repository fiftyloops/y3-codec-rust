use primitive::decoder::decode;
use primitive::encoder::PrimitivePacketEncoder;
use primitive::PrimitivePacket;

#[test]
fn float32() {
    let value: f32 = 1.0;
    let mut enc = PrimitivePacketEncoder::new(0x0A);
    enc.from_float32(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0A, 0x02, 0x3F, 0x80]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_float32() {
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
fn float64() {
    let value: f64 = 1.0;
    let mut enc = PrimitivePacketEncoder::new(0x0A);
    enc.from_float64(value);

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0A, 0x02, 0x3F, 0xF0]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            match pp.to_float64() {
                Ok(retval) => assert_eq!(retval, value),
                Err(msg) => panic!("{}", msg),
            }
            assert_eq!(state.pos,  4);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}
