use tag::Tag;

#[test]
fn node() {
    let value = 0x81;
    let tag = Tag {
        raw: value,
    };
    assert!(!tag.is_slice());
}

#[test]
fn slice() {
    let value = 0x42;
    let tag = Tag {
        raw: value,
    };
    assert!(tag.is_slice());
}
