use crate::errors::{DviousError, DviousResult};
use crate::util::num::{i24, u24};

pub struct ByteReader {
    position: usize,
    bytes: Vec<u8>,
}

pub trait Readable {
    fn from_u8_be(_: &[u8]) -> Self;
    fn size_in_bytes() -> usize;
}

impl ByteReader {
    pub fn new(bytes: Vec<u8>) -> ByteReader {
        ByteReader {
            position: 0,
            bytes: bytes,
        }
    }

    pub fn peek_be<T: Readable>(&self) -> DviousResult<T> {
        let number_of_bytes = T::size_in_bytes();
        let buffer = self.peek_slice(number_of_bytes)?;
        let result = T::from_u8_be(buffer);
        Ok(result)
    }

    pub fn read_be<T: Readable>(&mut self) -> DviousResult<T> {
        let number_of_bytes = T::size_in_bytes();
        let buffer = self.read_slice(number_of_bytes)?;
        let result = T::from_u8_be(buffer);
        Ok(result)
    }

    pub fn read_vector_be<T: Readable>(&mut self, k: usize) -> DviousResult<Vec<T>> {
        let number_of_bytes = T::size_in_bytes();
        let mut result = Vec::with_capacity(k);
        for _ in 0..k {
            let buffer = self.read_slice(number_of_bytes)?;
            let x = T::from_u8_be(buffer);
            result.push(x);
        }
        Ok(result)
    }

    pub fn skip_bytes<T: Into<u32>>(&mut self, k: T) -> DviousResult<()> {
        // Consume leftover bytes
        for _ in 0..k.into() {
            self.read_be::<u8>()?;
        }
        Ok(())
    }

    fn peek_slice(&self, n: usize) -> DviousResult<&[u8]> {
        let start = self.position;
        let end = self.position + n;

        if end <= self.bytes.len() {
            let result = &self.bytes[start..end];
            Ok(result)
        } else {
            Err(DviousError::IndexOutOfBoundsError)
        }
    }

    fn read_slice(&mut self, n: usize) -> DviousResult<&[u8]> {
        let start = self.position;
        let end = self.position + n;

        if end <= self.bytes.len() {
            let result = &self.bytes[start..end];
            self.position += n;
            Ok(result)
        } else {
            Err(DviousError::IndexOutOfBoundsError)
        }
    }

    pub fn has_more(&self) -> bool {
        self.position < self.bytes.len()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

// Unsigned

impl Readable for u8 {
    fn from_u8_be(b: &[u8]) -> Self {
        b[0]
    }

    fn size_in_bytes() -> usize {
        1
    }
}

impl Readable for u16 {
    fn from_u8_be(b: &[u8]) -> Self {
        u16::from(b[0]) << 8 | u16::from(b[1])
    }

    fn size_in_bytes() -> usize {
        2
    }
}

impl Readable for u24 {
    fn from_u8_be(b: &[u8]) -> Self {
        let result = u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]);
        u24::from(result)
    }

    fn size_in_bytes() -> usize {
        3
    }
}

impl Readable for u32 {
    fn from_u8_be(b: &[u8]) -> Self {
        u32::from(b[0]) << 24 | u32::from(b[1]) << 16 | u32::from(b[2]) << 8 | u32::from(b[3])
    }

    fn size_in_bytes() -> usize {
        4
    }
}

// Signed

impl Readable for i8 {
    fn from_u8_be(b: &[u8]) -> Self {
        b[0] as i8
    }

    fn size_in_bytes() -> usize {
        1
    }
}

impl Readable for i16 {
    fn from_u8_be(b: &[u8]) -> Self {
        i16::from(b[0]) << 8 | i16::from(b[1])
    }

    fn size_in_bytes() -> usize {
        2
    }
}

impl Readable for i24 {
    fn from_u8_be(b: &[u8]) -> Self {
        let result = i32::from(b[0]) << 16 | i32::from(b[1]) << 8 | i32::from(b[2]);
        i24::from(result)
    }

    fn size_in_bytes() -> usize {
        3
    }
}

impl Readable for i32 {
    fn from_u8_be(b: &[u8]) -> Self {
        i32::from(b[0]) << 24 | i32::from(b[1]) << 16 | i32::from(b[2]) << 8 | i32::from(b[3])
    }

    fn size_in_bytes() -> usize {
        4
    }
}

#[cfg(test)]
mod tests {
    use crate::util::byte_reader::{ByteReader, i24, u24};

    // Peek unsigned

    #[test]
    fn test_peek_u8_be() {
        let reader = get_reader(vec![0x42]);
        let result = reader.peek_be::<u8>().unwrap();

        assert_eq!(result, 0x42);
    }

    #[test]
    fn test_peek_u16_be() {
        let reader = get_reader(vec![0xDE, 0xAD]);
        let result = reader.peek_be::<u16>().unwrap();

        assert_eq!(result, 0xDEAD);
    }

    #[test]
    fn test_peek_u24_be() {
        let reader = get_reader(vec![0xDE, 0xAD, 0xBE]);
        let result = reader.peek_be::<u24>().unwrap();

        assert_eq!(result, u24::from(0xDEADBE));
    }

    #[test]
    fn test_peek_u32_be() {
        let reader = get_reader(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let result = reader.peek_be::<u32>().unwrap();

        assert_eq!(result, 0xDEADBEEF);
    }

    // Peek signed

    #[test]
    fn test_peek_i8_be() {
        let reader = get_reader(vec![0x42]);
        let result = reader.peek_be::<i8>().unwrap();

        assert_eq!(result, 0x42);
    }

    #[test]
    fn test_peek_i16_be() {
        let reader = get_reader(vec![0x1E, 0xAD]);
        let result = reader.peek_be::<i16>().unwrap();

        assert_eq!(result, 0x1EAD);
    }

    #[test]
    fn test_peek_i24_be() {
        let reader = get_reader(vec![0xDE, 0xAD, 0xBE]);
        let result = reader.peek_be::<i24>().unwrap();

        assert_eq!(result, i24::from(0xDEADBE));
    }

    #[test]
    fn test_peek_i32_be() {
        let reader = get_reader(vec![0x0E, 0xAD, 0xBE, 0xEF]);
        let result = reader.peek_be::<i32>().unwrap();

        assert_eq!(result, 0x0EADBEEF);
    }

    // Read unsigned

    #[test]
    fn test_read_u8_be() {
        let mut reader = get_reader(vec![0x42]);
        let result = reader.read_be::<u8>().unwrap();

        assert_eq!(result, 0x42);
    }

    #[test]
    fn test_read_u16_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD]);
        let result = reader.read_be::<u16>().unwrap();

        assert_eq!(result, 0xDEAD);
    }

    #[test]
    fn test_read_u24_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD, 0xBE]);
        let result = reader.read_be::<u24>().unwrap();

        assert_eq!(result, u24::from(0xDEADBE));
    }

    #[test]
    fn test_read_u32_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let result = reader.read_be::<u32>().unwrap();

        assert_eq!(result, 0xDEADBEEF);
    }

    // Read signed

    #[test]
    fn test_read_i8_be() {
        let mut reader = get_reader(vec![0x42]);
        let result = reader.read_be::<i8>().unwrap();

        assert_eq!(result, 0x42);
    }

    #[test]
    fn test_read_i16_be() {
        let mut reader = get_reader(vec![0x1E, 0xAD]);
        let result = reader.read_be::<i16>().unwrap();

        assert_eq!(result, 0x1EAD);
    }

    #[test]
    fn test_read_i24_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD, 0xBE]);
        let result = reader.read_be::<i24>().unwrap();

        assert_eq!(result, i24::from(0xDEADBE));
    }

    #[test]
    fn test_read_i32_be() {
        let mut reader = get_reader(vec![0x0E, 0xAD, 0xBE, 0xEF]);
        let result = reader.read_be::<i32>().unwrap();

        assert_eq!(result, 0x0EADBEEF);
    }

    // Read vector

    #[test]
    fn test_read_vector_u8() {
        let mut reader = get_reader(vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
        let result = reader.read_vector_be::<u8>(5).unwrap();

        assert_eq!(result, vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
    }

    #[test]
    fn test_read_vector_u16() {
        let mut reader = get_reader(vec![0xAA, 0xAA, 0xBB, 0xBB, 0xCC, 0xCC, 0xDD, 0xDD]);
        let result = reader.read_vector_be::<u16>(4).unwrap();

        assert_eq!(result, vec![0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]);
    }

    // Several reads

    #[test]
    fn test_read_several_be() {
        let mut reader = get_reader(vec![0xAA, 0xBB, 0xBB, 0xCC, 0xCC, 0xCC, 0xCC]);

        assert!(reader.has_more(), "Expected that reader has more");
        assert_eq!(reader.read_be::<u8>().unwrap(), 0xAA);

        assert!(reader.has_more(), "Expected that reader has more");
        assert_eq!(reader.read_be::<u16>().unwrap(), 0xBBBB);

        assert!(reader.has_more(), "Expected that reader has more");
        assert_eq!(reader.read_be::<u32>().unwrap(), 0xCCCCCCCC);

        assert!(!reader.has_more(), "Expected that reader has no more");
        assert!(reader.read_be::<u32>().is_err(), "Expected that 'e' is Err");
    }

    // Skip

    #[test]
    fn test_skip_bytes() {
        let mut reader = get_reader(vec![0xDE, 0xAD, 0xBE, 0xEF]);

        reader.skip_bytes(2_u32).unwrap();

        assert_eq!(reader.position(), 2);
    }

    // Has more

    #[test]
    fn test_has_more() {
        let reader = get_reader(vec![0xDE, 0xAD]);

        assert!(reader.has_more(), "Expected that reader has more");
    }

    #[test]
    fn test_has_no_more() {
        let mut reader = get_reader(vec![0xDE, 0xAD]);
        reader.read_be::<u16>().unwrap();

        assert!(!reader.has_more(), "Expected that reader has no more");
    }

    // Len

    #[test]
    fn test_len() {
        let reader = get_reader(vec![0xDE, 0xAD]);

        assert_eq!(reader.len(), 2_usize);
    }

    #[test]
    fn test_position() {
        let mut reader = get_reader(vec![0xDE, 0xAD]);

        reader.read_be::<u8>().unwrap();

        assert_eq!(reader.position(), 1_usize);
    }

    // Util

    fn get_reader(bytes: Vec<u8>) -> ByteReader {
        ByteReader::new(bytes)
    }
}
