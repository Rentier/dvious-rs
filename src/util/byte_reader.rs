use std::marker;
use std::mem::size_of;

use errors::{DviousError, DviousResult};

pub struct ByteReader {
    position: usize,
    bytes: Vec<u8>,
}

pub trait Readable: marker::Sized {
    fn from_u8_be(&[u8]) -> Self;
}

impl ByteReader {
    fn new(bytes: Vec<u8>) -> ByteReader {
        ByteReader {
            position: 0,
            bytes: bytes,
        }
    }

    pub fn peek_be<T: Readable>(&self) -> DviousResult<T> {
        let number_of_bytes = size_of::<T>();
        let buffer = self.peek_slice(number_of_bytes)?;
        let result = T::from_u8_be(buffer);
        Ok(result)
    }

    pub fn read_be<T: Readable>(&mut self) -> DviousResult<T> {
        let number_of_bytes = size_of::<T>();
        let buffer = self.read_slice(number_of_bytes)?;
        let result = T::from_u8_be(buffer);
        Ok(result)
    }

    fn peek_slice(&self, n: usize) -> Result<&[u8], DviousError> {
        let start = self.position;
        let end = self.position + n;

        println!("Start: {}, End: {}, Len: {}", start, end, self.bytes.len());


        if end <= self.bytes.len() {
            let result = &self.bytes[start..end];
            Ok(result)
        } else {
            Err(DviousError::IndexOutOfBoundsError)
        }
    }

    fn read_slice(&mut self, n: usize) -> Result<&[u8], DviousError> {
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

    fn has_more(&self) -> bool {
        self.position < self.bytes.len()
    }
}

impl Readable for u8 {
    fn from_u8_be(a: &[u8]) -> Self {
        a[0]
    }
}

impl Readable for u16 {
    fn from_u8_be(b: &[u8]) -> Self {
        u16::from(b[0]) << 8 | u16::from(b[1])
    }
}

impl Readable for u32 {
    fn from_u8_be(b: &[u8]) -> Self {
        u32::from(b[0]) << 24 | u32::from(b[1]) << 16 | u32::from(b[2]) << 8 | u32::from(b[3])
    }
}

#[cfg(test)]
mod tests {
    use util::byte_reader::ByteReader;

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
    fn test_peek_u32_be() {
        let reader = get_reader(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let result = reader.peek_be::<u32>().unwrap();

        assert_eq!(result, 0xDEADBEEF);
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
    fn test_read_u32_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let result = reader.read_be::<u32>().unwrap();

        assert_eq!(result, 0xDEADBEEF);
    }

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

    fn get_reader(bytes: Vec<u8>) -> ByteReader {
        ByteReader::new(bytes)
    }
}
