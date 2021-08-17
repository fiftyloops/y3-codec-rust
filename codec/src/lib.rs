pub mod tagged_int;

#[derive(Debug)]
pub struct Codec {
    pub ptr:  usize,
    pub size: usize,
}
