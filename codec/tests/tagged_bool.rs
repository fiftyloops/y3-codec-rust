use codec::Codec;

#[test]
fn bool() {
    test_bool(false, vec![0x00]);
    test_bool(true,  vec![0x01]);
}

fn test_bool(value: bool, bytes: Vec<u8>) {
    let size = bytes.len();
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    let mut codec = Codec {
        ptr:    0,
        size:   size,
    };
    match codec.encode_tagged_bool(&mut buffer, value) {
        Ok(_) => {
            for i in 0..size {
                assert_eq!(buffer[i], bytes[i]);
            }
        },
        Err(msg) => println!("{}", msg),
    }
    match codec.decode_tagged_bool(buffer) {
        Ok(decoder_output) => assert_eq!(decoder_output, value),
        Err(msg) => println!("{}", msg),
    }
}
