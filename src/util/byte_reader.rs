use errors::{DviousError, DviousResult};

pub struct ByteReader {
    position: usize,
    bytes: Vec<u8>,
}

impl ByteReader {
    fn new(bytes: Vec<u8>) -> ByteReader {
        ByteReader {
            position: 0,
            bytes: bytes,
        }
    }

    fn peek_slice(&self, n: usize) -> Result<&[u8], DviousError> {
        let start = self.position;
        let end = self.position + n;

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

    // Peek unsigned

    fn peek_u8_be(&self) -> DviousResult<u8> {
        let b = self.peek_slice(1)?;
        Ok(b[0])
    }

    fn peek_u16_be(&self) -> DviousResult<u16> {
        let b = self.peek_slice(2)?;
        let result = u16::from(b[0]) << 8 | u16::from(b[1]);
        Ok(result)
    }

    fn peek_u32_be(&self) -> DviousResult<u32> {
        let b = self.peek_slice(3)?;
        let result = u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]);
        Ok(result)
    }

    fn peek_u64_be(&self) -> DviousResult<u64> {
        let b = self.peek_slice(4)?;
        let result =
            u64::from(b[0]) << 24 | u64::from(b[1]) << 16 | u64::from(b[2]) << 8 | u64::from(b[3]);
        Ok(result)
    }    

    // Read unsigned

    fn read_u8_be(&mut self) -> DviousResult<u8> {
        let b = self.read_slice(1)?;
        Ok(b[0])
    }

    fn read_u16_be(&mut self) -> DviousResult<u16> {
        let b = self.read_slice(2)?;
        let result = u16::from(b[0]) << 8 | u16::from(b[1]);
        Ok(result)
    }

    fn read_u32_be(&mut self) -> DviousResult<u32> {
        let b = self.read_slice(3)?;
        let result = u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]);
        Ok(result)
    }

    fn read_u64_be(&mut self) -> DviousResult<u64> {
        let b = self.read_slice(4)?;
        let result =
            u64::from(b[0]) << 24 | u64::from(b[1]) << 16 | u64::from(b[2]) << 8 | u64::from(b[3]);
        Ok(result)
    }

    fn has_more(&self) -> bool {
        self.position < self.bytes.len()
    }

    // Read signed
}

#[cfg(test)]
mod tests {
    use util::byte_reader::ByteReader;

    // Peek unsigned

    #[test]
    fn test_peek_u8_be() {
        let mut reader = get_reader(vec![0x42]);
        let result = reader.peek_u8_be().unwrap();

        assert_eq!(result, 0x42);
    }

    #[test]
    fn test_peek_u16_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD]);
        let result = reader.peek_u16_be().unwrap();

        assert_eq!(result, 0xDEAD);
    }

    #[test]
    fn test_peek_u32_be() {
        let reader = get_reader(vec![0xDE, 0xAD, 0xBE]);
        let result = reader.peek_u32_be().unwrap();

        assert_eq!(result, 0xDEADBE);
    }

    #[test]
    fn test_peek_u64_be() {
        let reader = get_reader(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let result = reader.peek_u64_be().unwrap();

        assert_eq!(result, 0xDEADBEEF);
    }    

    // Read unsigned

    #[test]
    fn test_read_u8_be() {
        let mut reader = get_reader(vec![0x42]);
        let result = reader.read_u8_be().unwrap();

        assert_eq!(result, 0x42);
    }

    #[test]
    fn test_read_u16_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD]);
        let result = reader.read_u16_be().unwrap();

        assert_eq!(result, 0xDEAD);
    }

    #[test]
    fn test_read_u32_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD, 0xBE]);
        let result = reader.read_u32_be().unwrap();

        assert_eq!(result, 0xDEADBE);
    }

    #[test]
    fn test_read_u64_be() {
        let mut reader = get_reader(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let result = reader.read_u64_be().unwrap();

        assert_eq!(result, 0xDEADBEEF);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_read_several_be() {
        let mut reader = get_reader(vec![0xAA, 0xBB, 0xBB, 0xCC, 0xCC, 0xCC, 0xDD, 0xDD, 0xDD, 0xDD]);

        assert!(reader.has_more(), "Expected that reader has more");
        assert_eq!(reader.read_u8_be().unwrap(), 0xAA);

        assert!(reader.has_more(), "Expected that reader has more");
        assert_eq!(reader.read_u16_be().unwrap(), 0xBBBB);

        assert!(reader.has_more(), "Expected that reader has more");
        assert_eq!(reader.read_u32_be().unwrap(), 0xCCCCCC);

        assert!(reader.has_more(), "Expected that reader has more");
        assert_eq!(reader.read_u64_be().unwrap(), 0xDDDDDDDD);

        assert!(!reader.has_more(), "Expected that reader has no more");
        assert!(reader.read_u64_be().is_err(), "Expected that 'e' is Err");
    }

    #[test]
    fn test_has_more() {
        let mut reader = get_reader(vec![0xDE, 0xAD]);

        assert!(reader.has_more(), "Expected that reader has more");
    }

    #[test]
    fn test_has_no_more() {
        let mut reader = get_reader(vec![0xDE, 0xAD]);
        reader.read_u16_be();

        assert!(!reader.has_more(), "Expected that reader has no more");
    }

    fn get_reader(bytes: Vec<u8>) -> ByteReader {
        ByteReader::new(bytes)
    }
}
