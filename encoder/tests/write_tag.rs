use bytebuffer::ByteBuffer;
use encoder::Encoder;

#[test]
fn node() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_slice:   false,
        complete:   false,
    };
    encoder.write_tag();
    assert_eq!(encoder.seq_id, 0xaa);
}

#[test]
fn array() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    false,
        is_slice:   true,
        complete:   false,
    };
    encoder.write_tag();
    assert_eq!(encoder.seq_id, 0x6a);
}

#[test]
fn neither() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    false,
        is_slice:   false,
        complete:   false,
    };
    encoder.write_tag();
    assert_eq!(encoder.seq_id, 0x2a);
}

#[test]
fn both() {
    let mut encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_slice:   true,
        complete:   false,
    };
    encoder.write_tag();
    assert_eq!(encoder.seq_id, 0xea);
}

#[test]
#[should_panic]
fn error() {
    let mut encoder = Encoder {
        seq_id:     192,  // 11000000
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_slice:   true,
        complete:   false,
    };
    encoder.write_tag();
}
