pub mod tagged_varlen_bool;
pub mod tagged_varlen_int;
pub mod varlen_float;

#[derive(Debug)]
pub struct Codec {
    pub ptr:  usize,
    pub size: usize,
}
