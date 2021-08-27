use packet::tag::Tag;

#[test]
fn node() {
    let value = 0x81;
    let tag = Tag {
        raw: value,
    };
    assert!(tag.is_node());
    assert!(!tag.is_slice());
    assert_eq!(tag.raw(), value);
    assert_eq!(tag.seq_id(), 0x01);
}

#[test]
fn slice() {
    let value = 0x42;
    let tag = Tag {
        raw: value,
    };
    assert!(!tag.is_node());
    assert!(tag.is_slice());
    assert_eq!(tag.raw(), value);
    assert_eq!(tag.seq_id(), 0x02);
}
