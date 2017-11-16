use errors::{DviousError, DviousResult};
use util::byte_reader::ByteReader;

#[derive(Debug, PartialEq)]
pub struct TexFontMetric {
    header: TexFontMetricHeader,
}

#[derive(Debug, PartialEq)]
pub struct TexFontMetricHeader {
    checksum: u32,
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
        Ok(TexFontMetricHeader { checksum })
    }

    fn read_fixword(&mut self) -> DviousResult<f64> {
        let fixword_factor = 2.0_f64.powi(-20);
        let b = self.reader.read_be::<i32>()?;
        Ok(f64::from(b) * fixword_factor)
    }
}

#[cfg(test)]
mod tests {
    use fonts::tfm::*;
    use errors::DviousResult;

    #[test]
    fn test_read_header() {
        let mut tfm_reader = TexFontMetricReader::new(vec![0xAA, 0xBB, 0xCC, 0xDD]);

        let header = tfm_reader.read_header(4).unwrap();

        assert_eq!(
            header,
            TexFontMetricHeader {
                checksum: 0xAABBCCDD,
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
}
