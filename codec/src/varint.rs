use crate::Codec;

// int32

pub fn size_of_varint32(value: i32) -> usize {
    size_of_varint(value as i64, 32)
}

impl Codec {
    pub fn encode_varint32(&mut self, buffer: &mut Vec<u8>, value: i32) -> Result<(), String> {
        self.encode_varint(buffer, value as i64)
    }

    pub fn decode_varint32(&mut self, buffer: &Vec<u8>) -> Result<i32, String> {
        match self.decode_varint(&buffer) {
            Ok(value) => Ok(value as i32),
            Err(msg) => Err(msg),
        }
    }
}

// uint32

pub fn size_of_varuint32(value: u32) -> usize {
    size_of_varint(value as i32 as i64, 32)
}

impl Codec {
    pub fn encode_varuint32(&mut self, buffer: &mut Vec<u8>, value: u32) -> Result<(), String> {
        self.encode_varint(buffer, value as i32 as i64)
    }

    pub fn decode_varuint32(&mut self, buffer: &Vec<u8>) -> Result<u32, String> {
        match self.decode_varint(&buffer) {
            Ok(value) => Ok(value as u32),
            Err(msg) => Err(msg),
        }
    }
}

// int64

pub fn size_of_varint64(value: i64) -> usize {
    size_of_varint(value, 64)
}

impl Codec {
    pub fn encode_varint64(&mut self, buffer: &mut Vec<u8>, value: i64) -> Result<(), String> {
        self.encode_varint(buffer, value)
    }

    pub fn decode_varint64(&mut self, buffer: Vec<u8>) -> Result<i64, String> {
        self.decode_varint(&buffer)
    }
}

// uint64

pub fn size_of_varuint64(value: u64) -> usize {
    size_of_varint(value as i64, 64)
}

impl Codec {
    pub fn encode_varuint64(&mut self, buffer: &mut Vec<u8>, value: u64) -> Result<(), String> {
        self.encode_varint(buffer, value as i64)
    }

    pub fn decode_varuint64(&mut self, buffer: Vec<u8>) -> Result<u64, String> {
        match self.decode_varint(&buffer) {
            Ok(value) => Ok(value as u64),
            Err(msg) => Err(msg),
        }
    }
}

// general

fn size_of_varint(value: i64, bits: usize) -> usize {
    if bits != 32 && bits != 64 {
        panic!("bits must be either 32 or 64");
    }
    let prefix = value >> (bits - 1);
    let mut size = bits / 8 - 1;
    while size > 0 {
        let look_ahead = value >> (8 * size - 1);
        if look_ahead != prefix {
            return size + 1;
        }
        size -= 1;
    }
    1
}

impl Codec {
    pub fn encode_varint(&mut self, buffer: &mut Vec<u8>, value: i64) -> Result<(), String> {
        if self.size == 0 {
            return Err("nothing to encode".to_string());
        }
        while self.size > 0 {
            if self.ptr >= buffer.len() {
                return Err("insufficient buffer size".to_string());
            }
            self.size -= 1;
            buffer[self.ptr] = (value >> (8 * self.size)) as u8;
            self.ptr += 1;
        }
        Ok(())
    }

    pub fn decode_varint(&mut self, buffer: &Vec<u8>) -> Result<i64, String> {
        if self.size == 0 {
            return Err("nothing to encode".to_string().to_string());
        }
        let mut value: i64 = 0;
        if self.size > 0 {
            if self.ptr >= buffer.len() {
                return Err("insufficient buffer size".to_string().to_string());
            }
            value = (buffer[self.ptr] >> 7) as i64;
        }
        while self.size > 0 {
            if self.ptr >= buffer.len() {
                return Err("insufficient buffer size".to_string().to_string());
            }
            self.size -= 1;
            value = (value << 8) | (buffer[self.ptr]) as i64;
            self.ptr += 1;
        }
        Ok(value)
    }
}
