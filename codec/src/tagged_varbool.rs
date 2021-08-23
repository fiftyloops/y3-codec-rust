use crate::Codec;

impl Codec {
    pub fn encode_tagged_varbool(&mut self, buffer: &mut Vec<u8>, value: bool) -> Result<(), &str> {
        self.encode_tagged_varlen_int(buffer, if value { 1 as i64 } else { 0 as i64 })
    }

    pub fn decode_tagged_varbool(&mut self, buffer: Vec<u8>) -> Result<bool, &str> {
        match self.decode_tagged_varlen_int(buffer) {
            Ok(value) => Ok(value != 0),
            Err(msg) => Err(msg),
        }
    }
}
