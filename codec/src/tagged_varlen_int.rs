use crate::Codec;

// int32

pub fn size_of_tagged_varlen_int32(value: i32) -> usize {
    size_of_tagged_varlen_int(value as i64, 32)
}

impl Codec {
    pub fn encode_tagged_varlen_int32(&mut self, buffer: &mut Vec<u8>, value: i32) -> Result<(), &str> {
        self.encode_tagged_varlen_int(buffer, value as i64)
    }

    pub fn decode_tagged_varlen_int32(&mut self, buffer: Vec<u8>) -> Result<i32, &str> {
        match self.decode_tagged_varlen_int(buffer) {
            Ok(value) => Ok(value as i32),
            Err(msg) => Err(msg),
        }
    }
}

// uint32

pub fn size_of_tagged_varlen_uint32(value: u32) -> usize {
    size_of_tagged_varlen_int(value as i32 as i64, 32)
}

impl Codec {
    pub fn encode_tagged_varlen_uint32(&mut self, buffer: &mut Vec<u8>, value: u32) -> Result<(), &str> {
        self.encode_tagged_varlen_int(buffer, value as i32 as i64)
    }

    pub fn decode_tagged_varlen_uint32(&mut self, buffer: Vec<u8>) -> Result<u32, &str> {
        match self.decode_tagged_varlen_int(buffer) {
            Ok(value) => Ok(value as u32),
            Err(msg) => Err(msg),
        }
    }
}

// int64

pub fn size_of_tagged_varlen_int64(value: i64) -> usize {
    size_of_tagged_varlen_int(value, 64)
}

impl Codec {
    pub fn encode_tagged_varlen_int64(&mut self, buffer: &mut Vec<u8>, value: i64) -> Result<(), &str> {
        self.encode_tagged_varlen_int(buffer, value)
    }

    pub fn decode_tagged_varlen_int64(&mut self, buffer: Vec<u8>) -> Result<i64, &str> {
        self.decode_tagged_varlen_int(buffer)
    }
}

// uint64

pub fn size_of_tagged_varlen_uint64(value: u64) -> usize {
    size_of_tagged_varlen_int(value as i64, 64)
}

impl Codec {
    pub fn encode_tagged_varlen_uint64(&mut self, buffer: &mut Vec<u8>, value: u64) -> Result<(), &str> {
        self.encode_tagged_varlen_int(buffer, value as i64)
    }

    pub fn decode_tagged_varlen_uint64(&mut self, buffer: Vec<u8>) -> Result<u64, &str> {
        match self.decode_tagged_varlen_int(buffer) {
            Ok(value) => Ok(value as u64),
            Err(msg) => Err(msg),
        }
    }
}

// general

fn size_of_tagged_varlen_int(value: i64, bits: usize) -> usize {
    if bits != 32 && bits != 64 {
        panic!("bits must be either 32 or 64");
    }
    let prefix = value >> (bits - 1);
    let mut size = bits / 7;
    while size > 0 {
        let look_ahead = value >> (7 * size - 1);
        if look_ahead != prefix {
            return size + 1;
        }
        size -= 1;
    }
    1
}

impl Codec {
    pub fn encode_tagged_varlen_int(&mut self, buffer: &mut Vec<u8>, value: i64) -> Result<(), &str> {
        if self.size == 0 {
            return Err("nothing to encode");
        }
        while self.size != 0 && self.ptr < buffer.len() {
            self.size -= 1;
            let byte = value >> (7 * self.size);
            buffer[self.ptr] = (byte | 0x80) as u8;  // set most significant bit to 1
            self.ptr += 1;
        }
        if self.size != 0 {
            return Err("insufficient buffer size");
        }
        buffer[self.ptr - 1] &= 0x7F;
        Ok(())
    }

    pub fn decode_tagged_varlen_int(&mut self, buffer: Vec<u8>) -> Result<i64, &str> {
        if self.ptr >= buffer.len() {
            return Err("insufficient buffer size");
        }
        let mut value: i64 = 0;
        if self.size == 0 {  // initialize sign bit
            value = (buffer[self.ptr] as i64) << 1 >> 7;
        }
        while self.ptr < buffer.len() {
            let byte = buffer[self.ptr];
            self.ptr += 1;
            
            value <<= 7;
            value |= (byte & 0x7F) as i64;
            self.size += 1;
            if byte & 0x80 == 0 {  // last byte
                return Ok(value);
            }
        }
        return Err("insufficient buffer size");
    }
}
