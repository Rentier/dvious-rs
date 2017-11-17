use std::str;

use errors::{DviousError, DviousResult};
use util::byte_reader::ByteReader;

#[derive(Debug, PartialEq)]
pub struct TexFontMetric {
    header: TexFontMetricHeader,
}

#[derive(Debug, PartialEq)]
pub struct TexFontMetricHeader {
    checksum: u32,
    design_size: f64,
    encoding: Option<String>,
    font_identifier: Option<String>,
    face: Option<u8>,
    misc: Vec<u8>,
}

struct TexFontMetricReader {
    reader: ByteReader,
}

impl TexFontMetricReader {
    fn new(bytes: Vec<u8>) -> TexFontMetricReader {
        TexFontMetricReader {
            reader: ByteReader::new(bytes),
        }
    }

    fn read(&mut self) -> DviousResult<TexFontMetric> {
        // Length of the entire file
        let lf = self.reader.read_be::<u16>()? as usize;
        if lf != self.reader.len() {
            return Err(DviousError::TfmParseError(format!(
                "TFM specified to have {} bytes, but given were: {}",
                lf,
                self.reader.len()
            )));
        }

        // Header
        let lh = self.reader.read_be::<u16>()?;
        let header = self.read_header(lh)?;

        Ok(TexFontMetric { header })
    }

    fn read_header(&mut self, header_size: u16) -> DviousResult<TexFontMetricHeader> {
        let mut bytes_read = 0_u16;
        let checksum = self.reader.read_be::<u32>()?;
        bytes_read += 4;
        let design_size = self.read_fixword()?;
        bytes_read += 4;

        // Handle encoding information
        const ENCODING_FIELD_LENGTH : u16 = 39;
        let encoding_len = self.reader.read_be::<u8>()?;
        bytes_read += 1;
        if encoding_len > 39 {
            return Err(DviousError::TfmParseError(format!(
                "TFM header specified to have {} bytes, which is more than the allowed 0..39",
                encoding_len
            )));
        }
        
        let encoding = if bytes_read < header_size {
            let s = self.read_utf8_string(encoding_len as usize)?;
            bytes_read += ENCODING_FIELD_LENGTH;
            // Consume leftover bytes
            for _ in u16::from(encoding_len)..ENCODING_FIELD_LENGTH {
                self.reader.read_be::<u8>()?;
            }
            Option::Some(s)
        } else {
            Option::None
        };

        // Font identifier
        let font_identifier = if bytes_read < header_size {
            let s = self.read_utf8_string(20)?;
            bytes_read += 20;
            Option::Some(s)
        } else {
            Option::None
        };

        println!("Parsed font identifier");

        // Face
        let face = if bytes_read < header_size {
            self.reader.read_be::<u8>()?;
            self.reader.read_be::<u16>()?;
            println!("Has more: {}; Total: {}", self.reader.has_more(), self.reader.len());
            let face_byte = self.reader.read_be::<u8>()?;

            bytes_read += 4;
            Option::Some(face_byte)
        } else {
            Option::None
        };

        println!("Parsed font face");

        // Misc
        let mut misc = Vec::new();
        while bytes_read < header_size {
            println!("{}, {}, {}, {}", bytes_read, header_size, self.reader.len(), self.reader.has_more());
            let b = self.reader.read_be::<u8>()?;
            misc.push(b);
            bytes_read += 1;
        }

        Ok(TexFontMetricHeader {
            checksum,
            design_size,
            encoding,
            font_identifier,
            face,
            misc,
        })
    }

    fn read_fixword(&mut self) -> DviousResult<f64> {
        let fixword_factor = 2.0_f64.powi(-20);
        let b = self.reader.read_be::<i32>()?;
        Ok(f64::from(b) * fixword_factor)
    }

    fn read_utf8_string(&mut self, k: usize) -> DviousResult<String> {
        let vec = self.reader.read_vector_be::<u8>(k)?;
        let s = String::from_utf8(vec)?;
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use fonts::tfm::*;
    use errors::DviousResult;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_read_header() {
        let data = vec![
            0xAA, 0xBB, 0xCC, 0xDD,
            
            0x00, 0xA0, 0x00, 0x00,

            0x04, 0x54, 0x65, 0x73, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

            0x48, 0x45, 0x4c, 0x56, 0x45, 0x54, 0x49, 0x43, 0x41, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            
            0x00, 0x00, 0x00, 0x12,

            0xAA, 0xBB, 0xCC, 0xDD
        ];
        let bytes = data.len() as u16;
        let mut tfm_reader = TexFontMetricReader::new(data);

        let header = tfm_reader.read_header(bytes).unwrap();

        assert_eq!(
            header,
            TexFontMetricHeader {
                checksum: 0xAABBCCDD,
                design_size: 10.0,
                encoding: Some("Test".to_string()),
                font_identifier: Some("HELVETICA\0\0\0\0\0\0\0\0\0\0\0".to_string()),
                face: Option::Some(0x12),
                misc: vec![0xAA, 0xBB, 0xCC, 0xDD]
            }
        );
    }

    // Fixword

    #[test]
    fn test_read_fixword_min() {
        let mut tfm_reader = TexFontMetricReader::new(vec![0x80, 0x00, 0x00, 0x00]);

        let fixword = tfm_reader.read_fixword().unwrap();

        assert_eq!(fixword, -2048.0);
    }

    #[test]
    fn test_read_fixword_max() {
        let mut tfm_reader = TexFontMetricReader::new(vec![0x7F, 0xFF, 0xFF, 0xFF]);

        let fixword = tfm_reader.read_fixword().unwrap();

        assert_eq!(fixword, 2048.0 - 2.0_f64.powi(-20));
    }

    // Read string

    #[test]
    fn test_read_string() {
        let mut tfm_reader = TexFontMetricReader::new(vec![0x54, 0x65, 0x73, 0x74]);

        let result = tfm_reader.read_utf8_string(4).unwrap();

        assert_eq!(result, "Test".to_string());
    }
}
