use bytebuffer::ByteBuffer;
use encoder::Encoder;

#[test]
fn empty() {
    let encoder = Encoder {
        seq_id:     42,
        byte_array: vec![],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_array:   false,
        complete:   false,
    };
    assert!(encoder.is_empty());
}

#[test]
fn nonempty() {
    let encoder = Encoder {
        seq_id:     42,
        byte_array: vec![0xca, 0xfe],
        buffer:     ByteBuffer::new(),
        is_node:    true,
        is_array:   false,
        complete:   false,
    };
    assert!(!encoder.is_empty());
}
