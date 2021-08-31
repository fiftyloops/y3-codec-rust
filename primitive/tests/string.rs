use primitive::decoder::decode;
use primitive::encoder::PrimitivePacketEncoder;
use primitive::PrimitivePacket;

#[test]
fn empty() {
    let value: &str = "";
    let mut enc = PrimitivePacketEncoder::new(0x0C);
    enc.from_utf8_string(value.to_string());

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0C, 0x00]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            assert_eq!(pp.meta.byte_array.len(), 0);
            assert_eq!(pp.to_utf8_string(), value.to_string());
            assert_eq!(state.pos,  2);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}

#[test]
fn nonempty() {
    let value: &str = "yomo";
    let mut enc = PrimitivePacketEncoder::new(0x0B);
    enc.from_utf8_string(value.to_string());

    let buffer = enc.meta.encode();
    assert_eq!(buffer, vec![0x0B, 0x04, 0x79, 0x6F, 0x6D, 0x6F]);

    let mut pp: PrimitivePacket = Default::default();
    match decode(&buffer, &mut pp) {
        Ok(state) => {
            assert_eq!(buffer, pp.meta.raw_bytes());
            assert_eq!(pp.meta.byte_array.len(), 4);
            assert_eq!(pp.meta.byte_array[0], 0x79);
            assert_eq!(pp.meta.byte_array[1], 0x6F);
            assert_eq!(pp.meta.byte_array[2], 0x6D);
            assert_eq!(pp.meta.byte_array[3], 0x6F);
            assert_eq!(pp.to_utf8_string(), value.to_string());
            assert_eq!(state.pos,  6);
            assert_eq!(state.size, 1);
        }
        Err(msg) => panic!("{}", msg),
    }
}
