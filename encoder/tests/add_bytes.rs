use bytebuffer::ByteBuffer;
use encoder::Encoder;

#[test]
fn zero() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_array:   false,
        complete:   false,
    };
    encoder.add_bytes(&mut vec![]);
    let byte_array = encoder.get_bytes();
    assert_eq!(byte_array.len(), 2);
    assert_eq!(byte_array[0], 0xca);
    assert_eq!(byte_array[1], 0xfe);
}

#[test]
fn one() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_array:   false,
        complete:   false,
    };
    encoder.add_bytes(&mut vec![0xf0]);
    let byte_array = encoder.get_bytes();
    assert_eq!(byte_array.len(), 3);
    assert_eq!(byte_array[0], 0xca);
    assert_eq!(byte_array[1], 0xfe);
    assert_eq!(byte_array[2], 0xf0);
}

#[test]
fn two() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_array:   false,
        complete:   false,
    };
    encoder.add_bytes(&mut vec![0xf0, 0x0d]);
    let byte_array = encoder.get_bytes();
    assert_eq!(byte_array.len(), 4);
    assert_eq!(byte_array[0], 0xca);
    assert_eq!(byte_array[1], 0xfe);
    assert_eq!(byte_array[2], 0xf0);
    assert_eq!(byte_array[3], 0x0d);
}
