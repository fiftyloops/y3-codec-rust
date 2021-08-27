use bytebuffer::ByteBuffer;
use encoder::Encoder;

#[test]
fn zero() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_slice:   false,
        complete:   false,
    };
    encoder.write_len();
    let byte_array = encoder.buffer.to_bytes();
    assert_eq!(byte_array.len(), 1);
    assert_eq!(byte_array[0], 0x00);
}

#[test]
fn one() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_slice:   false,
        complete:   false,
    };
    encoder.write_len();
    let byte_array = encoder.buffer.to_bytes();
    assert_eq!(byte_array.len(), 1);
    assert_eq!(byte_array[0], 0x01);
}

#[test]
fn two() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_slice:   false,
        complete:   false,
    };
    encoder.write_len();
    let byte_array = encoder.buffer.to_bytes();
    assert_eq!(byte_array.len(), 1);
    assert_eq!(byte_array[0], 0x02);
}
