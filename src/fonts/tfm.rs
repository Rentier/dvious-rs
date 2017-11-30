use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use errors::{DviousError, DviousResult};
use util::byte_reader::ByteReader;

type Fixword = f64;

#[derive(Debug, PartialEq)]
pub struct TexFontMetric {
    header: TexFontMetricHeader,
    char_info: Vec<TexFontCharInfo>,
    width_table: Vec<Fixword>,
}

#[derive(Debug, PartialEq)]
pub struct TexFontMetricHeader {
    checksum: u32,
    design_size: Fixword,
    encoding: Option<String>,
    font_identifier: Option<String>,
    face: Option<u8>,
    misc: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct TexFontCharInfo {
    character: u16,
    width_index: u8,
    height_index: u8,
    depth_index: u8,
    italic_index: u8,
    tag: TexFontCharInfoTag,
}

#[derive(Debug, PartialEq)]
pub enum TexFontCharInfoTag {
    None,
    Ligature(u8),
    List(u8),
    Extensible(u8),
}

struct TexFontMetricReader {
    reader: ByteReader,
}

pub fn read_tfm_from_file(path: String) -> DviousResult<TexFontMetric> {
    let mut buffer = Vec::new();
    let mut file = File::open(&path)?;
    file.read_to_end(&mut buffer)?;
    let mut tfm_reader = TexFontMetricReader::new(buffer);
    tfm_reader.read()
}

#[allow(dead_code)] 
impl TexFontMetricReader {
    fn new(bytes: Vec<u8>) -> TexFontMetricReader {
        TexFontMetricReader {
            reader: ByteReader::new(bytes),
        }
    }

    fn read(&mut self) -> DviousResult<TexFontMetric> {
        // Length of the entire file
        let lf = self.reader.read_be::<u16>()? as usize;
        if lf * 4 != self.reader.len() {
            return Err(DviousError::TfmParseError(format!(
                "TFM specified to have {} bytes, but given were: {}",
                lf,
                self.reader.len()
            )));
        }

        let lh = self.reader.read_be::<u16>()?; // Length of header data
        let bc = self.reader.read_be::<u16>()?; // Smallest character code
        let ec = self.reader.read_be::<u16>()?; // Largest character code
        let nw = self.reader.read_be::<u16>()?; // Number of entries in width table
        let nh = self.reader.read_be::<u16>()?; // Number of entries in height table
        let nd = self.reader.read_be::<u16>()?; // Number of entries in depth table
        let ni = self.reader.read_be::<u16>()?; // Number of entries italic correction table
        let nl = self.reader.read_be::<u16>()?; // Number of entries in lig/kern table
        let nk = self.reader.read_be::<u16>()?; // Number of entries in kern table
        let ne = self.reader.read_be::<u16>()?; // Number of entries in extensible characters table
        let np = self.reader.read_be::<u16>()?; // Number of font parameters

        let header = self.read_header(lh)?;
        let char_info = self.read_char_info(bc, ec)?;
        let width_table = self.read_fixword_table(nw)?;

        Ok(TexFontMetric { header, char_info, width_table })
    }

    fn read_header(&mut self, header_size: u16) -> DviousResult<TexFontMetricHeader> {
        let mut bytes_read = 0_u16;
        let checksum = self.reader.read_be::<u32>()?;
        bytes_read += 4;
        let design_size = self.read_fixword()?;
        bytes_read += 4;

        // Handle encoding information
        const ENCODING_FIELD_LENGTH: u16 = 39;
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

        // Face
        let face = if bytes_read < header_size {
            self.reader.read_be::<u8>()?;
            self.reader.read_be::<u16>()?;
            let face_byte = self.reader.read_be::<u8>()?;

            bytes_read += 4;
            Option::Some(face_byte)
        } else {
            Option::None
        };

        // Misc
        let mut misc = Vec::new();
        while bytes_read < header_size {
            println!(
                "{}, {}, {}, {}",
                bytes_read,
                header_size,
                self.reader.len(),
                self.reader.has_more()
            );
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

    fn read_char_info(&mut self, bc: u16, ec: u16) -> DviousResult<Vec<TexFontCharInfo>> {
        let mut result = Vec::new();

        for character in bc..ec + 1 {
            let first_byte = self.reader.read_be::<u8>()?;
            let second_byte = self.reader.read_be::<u8>()?;
            let third_byte = self.reader.read_be::<u8>()?;
            let fourth_byte = self.reader.read_be::<u8>()?;

            let width_index = first_byte;
            let height_index = (second_byte >> 4) * 16;
            let depth_index = second_byte & 0x0F;
            let italic_index = (third_byte >> 2) * 4;
            let tag_value = third_byte & 0b0000_0011;
            let remainder = fourth_byte;

            let tag = self.read_char_info_tag(tag_value, remainder)?;

            let char_info = TexFontCharInfo {
                character,
                width_index,
                height_index,
                depth_index,
                italic_index,
                tag,
            };
            result.push(char_info);
        }

        Ok(result)
    }

    fn read_char_info_tag(&self, tag_value: u8, remainder: u8) -> DviousResult<TexFontCharInfoTag> {
        let tag = match tag_value {
            0 => TexFontCharInfoTag::None,
            1 => TexFontCharInfoTag::Ligature(remainder),
            2 => TexFontCharInfoTag::List(remainder),
            3 => TexFontCharInfoTag::Extensible(remainder),
            _ => {
                return Err(DviousError::TfmParseError(format!(
                    "TFM character information specified invalid tag: {}",
                    tag_value
                )))
            }
        };
        Ok(tag)
    }

    fn read_fixword(&mut self) -> DviousResult<Fixword> {
        let f64_factor = 2.0_f64.powi(-20);
        let b = self.reader.read_be::<i32>()?;
        Ok(f64::from(b) * f64_factor)
    }

    fn read_fixword_table(&mut self, number_of_fixwords: u16) -> DviousResult<Vec<Fixword>> {
        let n = number_of_fixwords as usize;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let fixword = self.read_fixword()?;
            result.push(fixword);
        }
        Ok(result)
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
        let number_of_bytes = data.len() as u16;
        let mut tfm_reader = TexFontMetricReader::new(data);

        let header = tfm_reader.read_header(number_of_bytes).unwrap();

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

    // Char info

    #[test]
    fn test_read_charinfo() {
        let data = vec![0x42, 0xAB, 0b101010_10, 0xCD, 0x23, 0xCD, 0b010101_01, 0xEF];
        let mut tfm_reader = TexFontMetricReader::new(data);

        let char_info = tfm_reader.read_char_info(0x60, 0x61).unwrap();

        assert_eq!(
            char_info,
            vec![
                TexFontCharInfo {
                    character: 0x60,
                    width_index: 0x42,
                    height_index: 0xA * 16,
                    depth_index: 0xB,
                    italic_index: 42 * 4,
                    tag: TexFontCharInfoTag::List(0xCD),
                },
                TexFontCharInfo {
                    character: 0x61,
                    width_index: 0x23,
                    height_index: 0xC * 16,
                    depth_index: 0xD,
                    italic_index: 21 * 4,
                    tag: TexFontCharInfoTag::Ligature(0xEF),
                },
            ]
        );
    }

    #[test]
    fn test_read_charinfo_tag_none() {
        let data = vec![];
        let tfm_reader = TexFontMetricReader::new(data);

        let tag = tfm_reader.read_char_info_tag(0, 0x42).unwrap();

        assert_eq!(tag, TexFontCharInfoTag::None);
    }

    #[test]
    fn test_read_charinfo_tag_ligature() {
        let data = vec![];
        let tfm_reader = TexFontMetricReader::new(data);

        let tag = tfm_reader.read_char_info_tag(1, 0x42).unwrap();

        assert_eq!(tag, TexFontCharInfoTag::Ligature(0x42));
    }

    #[test]
    fn test_read_charinfo_tag_list() {
        let data = vec![];
        let tfm_reader = TexFontMetricReader::new(data);

        let tag = tfm_reader.read_char_info_tag(2, 0x42).unwrap();

        assert_eq!(tag, TexFontCharInfoTag::List(0x42));
    }

    #[test]
    fn test_read_charinfo_tag_extensible() {
        let data = vec![];
        let tfm_reader = TexFontMetricReader::new(data);

        let tag = tfm_reader.read_char_info_tag(3, 0x42).unwrap();

        assert_eq!(tag, TexFontCharInfoTag::Extensible(0x42));
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

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_read_fixword_table() {
        let data = vec![
            0x80, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00, 
            0x00, 0x0A, 0xBC, 0x00, 
        ];
        let mut tfm_reader = TexFontMetricReader::new(data);

        let fixword_table = tfm_reader.read_fixword_table(3).unwrap();

        assert_eq!(fixword_table, vec![-2048.0, 0.0, 0.6708984375]);
    }

    // Read string

    #[test]
    fn test_read_string() {
        let mut tfm_reader = TexFontMetricReader::new(vec![0x54, 0x65, 0x73, 0x74]);

        let result = tfm_reader.read_utf8_string(4).unwrap();

        assert_eq!(result, "Test".to_string());
    }
}
