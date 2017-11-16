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
    encoding: String,
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
        let checksum = self.reader.read_be::<u32>()?;
        let design_size = self.read_fixword()?;

        // Handle encoding information
        let encoding_len = self.reader.read_be::<u8>()?;
        if encoding_len > 39 {
            return Err(DviousError::TfmParseError(format!(
                "TFM header specified to have {} bytes, which is more than the allowed 0..39",
                encoding_len
            )));
        }
        let encoding = self.read_utf8_string(encoding_len as usize)?;

        Ok(TexFontMetricHeader {
            checksum,
            design_size,
            encoding,
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
            0x04, 0x54, 0x65, 0x73, 0x74,
        ];
        let bytes = data.len() as u16;
        let mut tfm_reader = TexFontMetricReader::new(data);

        let header = tfm_reader.read_header(bytes).unwrap();

        assert_eq!(
            header,
            TexFontMetricHeader {
                checksum: 0xAABBCCDD,
                design_size: 10.0,
                encoding: "Test".to_string(),
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
