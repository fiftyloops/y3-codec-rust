use bytebuffer::ByteBuffer;
use encoder::Encoder;

#[test]
fn empty() {
    let encoder = Encoder {
        seq_id: 42,
        byte_array: vec![],
        buffer: ByteBuffer::new(),
        is_node: true,
        is_slice: false,
        complete: false,
    };
    let bytes = encoder.get_bytes();
    assert_eq!(bytes.len(), 0);
}

#[test]
fn nonempty() {
    let encoder = Encoder {
        seq_id: 42,
        byte_array: vec![0xca, 0xfe],
        buffer: ByteBuffer::new(),
        is_node: true,
        is_slice: false,
        complete: false,
    };
    let bytes = encoder.get_bytes();
    assert_eq!(bytes.len(), 2);
    assert_eq!(bytes[0], 0xca);
    assert_eq!(bytes[1], 0xfe);
}
