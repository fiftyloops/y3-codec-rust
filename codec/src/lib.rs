pub mod tagged_varbool;
pub mod tagged_varint;
pub mod varfloat;
pub mod varint;

#[derive(Debug, Default)]
pub struct Codec {
    pub ptr: usize,
    pub size: usize,
}
