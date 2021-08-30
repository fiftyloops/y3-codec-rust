use codec::varint;
use codec::Codec;

#[test]
fn int32() {
    test_int32(-1, vec![0xFF]);
    test_int32(-5, vec![0xFB]);
    test_int32(63, vec![0x3F]);
    test_int32(-65, vec![0xBF]);
    test_int32(127, vec![0x7F]);
    test_int32(255, vec![0x00, 0xFF]);
    test_int32(-4097, vec![0xEF, 0xFF]);
    test_int32(-8193, vec![0xDF, 0xFF]);
    test_int32(-2097152, vec![0xE0, 0x00, 0x00]);
    test_int32(-134217729, vec![0xF7, 0xFF, 0xFF, 0xFF]);
    test_int32(-2147483648, vec![0x80, 0x00, 0x00, 0x00]);
}

#[test]
fn uint32() {
    test_uint32(1, vec![0x01]);
    test_uint32(127, vec![0x7F]);
    test_uint32(128, vec![0x00, 0x80]);
    test_uint32(130, vec![0x00, 0x82]);
    test_uint32(1048576, vec![0x10, 0x00, 0x00]);
    test_uint32(134217728, vec![0x08, 0x00, 0x00, 0x00]);
    test_uint32(4294967295, vec![0xFF]);
}

#[test]
fn int64() {
    test_int64(0, vec![0x00]);
    test_int64(1, vec![0x01]);
    test_int64(-1, vec![0xFF]);
}

#[test]
fn uint64() {
    test_uint64(0, vec![0x00]);
    test_uint64(1, vec![0x01]);
    test_uint64(18446744073709551615, vec![0xFF]);
}

fn test_int32(value: i32, bytes: Vec<u8>) {
    let size = varint::size_of_varint32(value);
    assert_eq!(size, bytes.len());

    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    let mut codec = Codec { ptr: 0, size: size };
    match codec.encode_varint32(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        }
        Err(msg) => println!("{}", msg),
    }
    match codec.decode_varint32(&buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => println!("{}", msg),
    }
}

fn test_uint32(value: u32, bytes: Vec<u8>) {
    let size = varint::size_of_varuint32(value);
    assert_eq!(size, bytes.len());

    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    let mut codec = Codec { ptr: 0, size: size };
    match codec.encode_varuint32(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        }
        Err(msg) => println!("{}", msg),
    }
    match codec.decode_varuint32(buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => println!("{}", msg),
    }
}

fn test_int64(value: i64, bytes: Vec<u8>) {
    let size = varint::size_of_varint64(value);
    assert_eq!(size, bytes.len());

    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    let mut codec = Codec { ptr: 0, size: size };
    match codec.encode_varint64(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        }
        Err(msg) => println!("{}", msg),
    }
    match codec.decode_varint64(buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => println!("{}", msg),
    }
}

fn test_uint64(value: u64, bytes: Vec<u8>) {
    let size = varint::size_of_varuint64(value);
    assert_eq!(size, bytes.len());

    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    let mut codec = Codec { ptr: 0, size: size };
    match codec.encode_varuint64(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        }
        Err(msg) => println!("{}", msg),
    }
    match codec.decode_varuint64(buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => println!("{}", msg),
    }
}
