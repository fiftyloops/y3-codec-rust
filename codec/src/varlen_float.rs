use crate::Codec;

// float32 (1 bit for the sign, 8 bits for the exponent, 23 bits for the mantissa)

pub fn size_of_varlen_float32(value: f32) -> usize {
    size_of_varlen_float(value.to_bits() as u64, 4)
}

impl Codec {
    pub fn encode_varlen_float32(&mut self, buffer: &mut Vec<u8>, value: f32) -> Result<(), &str> {
        self.encode_varlen_float(buffer, value.to_bits() as u64, 4)
    }

    pub fn decode_varlen_float32(&mut self, buffer: Vec<u8>) -> Result<f32, &str> {
        match self.decode_varlen_float(buffer, 4) {
            Ok(value) => Ok(f32::from_bits(value as u32)),
            Err(msg) => Err(msg),
        }
    }
}

// float64 (1 bit for the sign, 11 bits for the exponent, 52 bits for the mantissa)

pub fn size_of_varlen_float64(value: f64) -> usize {
    size_of_varlen_float(value.to_bits(), 8)
}

impl Codec {
    pub fn encode_varlen_float64(&mut self, buffer: &mut Vec<u8>, value: f64) -> Result<(), &str> {
        self.encode_varlen_float(buffer, value.to_bits(), 8)
    }

    pub fn decode_varlen_float64(&mut self, buffer: Vec<u8>) -> Result<f64, &str> {
        match self.decode_varlen_float(buffer, 8) {
            Ok(value) => Ok(f64::from_bits(value)),
            Err(msg) => Err(msg),
        }
    }
}

// general

fn size_of_varlen_float(value: u64, mut bytes: usize) -> usize {
    if bytes != 4 && bytes != 8 {
        panic!("bytes must be either 4 or 8");
    }
    let mask = 0xFF as u64;
    let mut n = 0;
    while bytes > 1 {
        if value & (mask << n) != 0 {
            return bytes;
        }
        bytes -= 1;
        n += 8;
    }
    1
}

impl Codec {
    pub fn encode_varlen_float(&mut self, buffer: &mut Vec<u8>, value: u64, bytes: usize) -> Result<(), &str> {
        if self.size == 0 {
            return Err("nothing to encode");
        }
        let (gap, mask) = self.size_of_gap(bytes);
        while (self.size & mask) > 0 {
            if self.ptr >= buffer.len() {
                return Err("insufficient buffer size");
            }
            self.size -= 1;
            buffer[self.ptr] = (value >> ((self.size & mask + gap) * 8)) as u8;
            self.ptr += 1;
        }
        self.size = 0;
        Ok(())
    }

    pub fn decode_varlen_float(&mut self, buffer: Vec<u8>, bytes: usize) -> Result<u64, &str> {
        if self.size == 0 {
            return Err("nothing to encode");
        }
        let mut value: u64 = 0;
        let (gap, mask) = self.size_of_gap(bytes);
        while (self.size & mask) > 0 {
            if self.ptr >= buffer.len() {
                return Err("insufficient buffer size");
            }
            self.size -= 1;
            value = (value << 8) | (buffer[self.ptr] as u64);
            self.ptr += 1;
        }
        value <<= gap * 8;
        self.size = 0;
        Ok(value)
    }

    fn size_of_gap(&mut self, bytes: usize) -> (usize, usize) {
        let n = (0 as usize).count_zeros();
        let mask = (1 << (n - 8)) - 1;  // keep the last (n - 8) bits

        let mut gap = 0;
        if self.size > 0 {
            if bytes > self.size {
                gap = bytes - self.size;
            }
            let sign = !0 << (n - 1);
            self.size = sign | (gap << (n - 8)) | (self.size & mask);
        } else {
            gap = (self.size >> (n - 8)) & 0x7F;
        }
        (gap, mask)
    }
}
