pub mod tagged_bool;
pub mod tagged_int;
pub mod varlen_float;

#[derive(Debug)]
pub struct Codec {
    pub ptr:  usize,
    pub size: usize,
}
