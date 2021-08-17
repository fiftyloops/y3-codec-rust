use codec::Codec;
use codec::tagged_int;

#[test]
fn int32() {
    test_int32(-1, vec![0x7F]);
    test_int32(-5, vec![0x7B]);
    test_int32(63, vec![0x3F]);
    test_int32(-65, vec![0xFF, 0x3F]);
    test_int32(127, vec![0x80, 0x7F]);
    test_int32(255, vec![0x81, 0x7F]);
    test_int32(-4097, vec![0xDF, 0x7F]);
    test_int32(-8193, vec![0xFF, 0xBF, 0x7F]);
    test_int32(-2097152, vec![0xFF, 0x80, 0x80, 0x00]);
    test_int32(-134217729, vec![0xFF, 0xBF, 0xFF, 0xFF, 0x7F]);
    test_int32(-2147483648, vec![0xF8, 0x80, 0x80, 0x80, 0x00]);
}

fn test_int32(value: i32, bytes: Vec<u8>) {
    let size = tagged_int::size_of_tagged_int32(value);
    assert_eq!(size, bytes.len());
    
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    let mut codec = Codec {
        ptr:    0,
        size:   size,
    };
    match codec.encode_tagged_int32(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        },
        Err(msg) => println!("{}", msg),
    }
    match codec.decode_tagged_int32(buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => println!("{}", msg),
    }
}
