use codec::varfloat;
use codec::Codec;

#[test]
fn float32() {
    test_float32(0.0, vec![0x00]);
    test_float32(1.0, vec![0x3F, 0x80]);
    test_float32(25.0, vec![0x41, 0xC8]);
    test_float32(-2.0, vec![0xC0]);
    test_float32(0.25, vec![0x3E, 0x80]);
    test_float32(0.375, vec![0x3E, 0xC0]);
    test_float32(12.375, vec![0x41, 0x46]);
    test_float32(68.123, vec![0x42, 0x88, 0x3E, 0xFA]);
}

#[test]
fn float64() {
    test_float64(0.0, vec![0x00]);
    test_float64(1.0, vec![0x3F, 0xF0]);
    test_float64(2.0, vec![0x40]);
    test_float64(23.0, vec![0x40, 0x37]);
    test_float64(-2.0, vec![0xC0]);
    test_float64(0.01171875, vec![0x3F, 0x88]);
}

fn test_float32(value: f32, bytes: Vec<u8>) {
    let size = varfloat::size_of_varfloat32(value);
    assert_eq!(size, bytes.len());

    let mut buffer: Vec<u8> = vec![0; size];
    let mut codec = Codec { ptr: 0, size: size };
    match codec.encode_varfloat32(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        }
        Err(msg) => panic!("{}", msg),
    }
    let mut codec = Codec { ptr: 0, size: size };
    match codec.decode_varfloat32(&buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => panic!("{}", msg),
    }
}

fn test_float64(value: f64, bytes: Vec<u8>) {
    let size = varfloat::size_of_varfloat64(value);
    assert_eq!(size, bytes.len());

    let mut buffer: Vec<u8> = vec![0; size];
    let mut codec = Codec { ptr: 0, size: size };
    match codec.encode_varfloat64(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        }
        Err(msg) => panic!("{}", msg),
    }
    let mut codec = Codec { ptr: 0, size: size };
    match codec.decode_varfloat64(&buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => panic!("{}", msg),
    }
}
