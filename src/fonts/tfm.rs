use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::errors::{DviousError, DviousResult};
use crate::util::byte_reader::ByteReader;
use crate::util::num::Fixword;

#[derive(Debug, PartialEq)]
pub struct TexFontMetric {
    pub header: TfmMetricHeader,
    pub char_info_table: HashMap<u8, TfmCharInfo>,
    pub width_table: Vec<Fixword>,
    pub heigth_table: Vec<Fixword>,
    pub depth_table: Vec<Fixword>,
    pub italic_table: Vec<Fixword>,
    pub lig_kern_table: Vec<TfmLigatureCommand>,
    pub kern_table: Vec<Fixword>,
    pub extension_table: Vec<TfmExtensionRecipe>,
    pub param_table: Vec<Fixword>,
}

#[derive(Debug, PartialEq)]
pub struct TfmMetricHeader {
    pub checksum: u32,
    pub design_size: Fixword,
    pub encoding: Option<String>,
    pub font_identifier: Option<String>,
    pub face: Option<u8>,
    pub misc: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct TfmCharInfo {
    pub character: u8,
    pub width_index: u8,
    pub height_index: u8,
    pub depth_index: u8,
    pub italic_index: u8,
    pub tag: TfmCharInfoTag,
}

#[derive(Debug, PartialEq)]
pub enum TfmCharInfoTag {
    None,
    Ligature(u8),
    List(u8),
    Extensible(u8),
}

#[derive(Debug, PartialEq)]
pub struct TfmLigatureCommand {
    pub skip_byte: u8,
    pub next_char: u8,
    pub op_byte: u8,
    pub remainder: u8,
}

#[derive(Debug, PartialEq)]
pub struct TfmExtensionRecipe {
    pub top: u8,
    pub mid: u8,
    pub bot: u8,
    pub rep: u8,
}

struct TfmMetricReader {
    reader: ByteReader,
}

pub fn read_tfm_from_file(path: String) -> DviousResult<TexFontMetric> {
    let mut buffer = Vec::new();
    let mut file = File::open(&path)?;
    file.read_to_end(&mut buffer)?;
    let mut tfm_reader = TfmMetricReader::new(buffer);
    tfm_reader.read()
}

impl TfmMetricReader {
    fn new(bytes: Vec<u8>) -> TfmMetricReader {
        TfmMetricReader {
            reader: ByteReader::new(bytes),
        }
    }

    fn read(&mut self) -> DviousResult<TexFontMetric> {
        // Length of the entire file in bytes, lf itself is in words (1 word = 4 bytes)
        let lf = 4 * self.reader.read_be::<u16>()? as usize;
        if lf != self.reader.len() {
            return Err(DviousError::TfmParseError(format!(
                "TFM specified to have [{}] bytes, but file on disk contains [{}] bytes",
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

        // Sanity checks
        if bc > ec {
            return Err(DviousError::TfmParseError(format!(
                "The character code range [{}]..[{}] is illegal!",
                bc,
                ec
            )));
        }

        let sum_of_lengths = (6 + lh + (ec - bc + 1) + nw + nh + nd + ni + nl + nk + ne + np) * 4;
        if lf != sum_of_lengths as usize {
            return Err(DviousError::TfmParseError(format!(
                "File says to contain [{}] bytes, but summing individual lengths adds up to [{}]",
                lf,
                sum_of_lengths
            )));
        }

        // Parsing

        let header = self.read_header(lh)?;
        let char_info_table = self.read_char_info_table(bc as u8, ec as u8)?;
        let width_table = self.read_fixword_table(nw)?;
        let heigth_table = self.read_fixword_table(nh)?;
        let depth_table = self.read_fixword_table(nd)?;
        let italic_table = self.read_fixword_table(ni)?;
        let lig_kern_table = self.read_lig_kern_table(nl)?;
        let kern_table = self.read_fixword_table(nk)?;
        let extension_table = self.read_extension_table(ne)?;
        let param_table = self.read_fixword_table(np)?;

        debug_assert!(
            !self.reader.has_more(),
            "Expected reader to reach end of input, but only read [{}/{}] bytes",
            self.reader.position(),
            self.reader.len(),
        );

        Ok(TexFontMetric {
            header,
            char_info_table,
            width_table,
            heigth_table,
            depth_table,
            italic_table,
            lig_kern_table,
            kern_table,
            extension_table,
            param_table,
        })
    }

    /// `header_size` is given in words, not in bytes
    fn read_header(&mut self, header_size: u16) -> DviousResult<TfmMetricHeader> {
        let mut words_read = 0_u16;
        let checksum = self.reader.read_be::<u32>()?;
        words_read += 1;
        let design_size = self.read_fixword()?;
        words_read += 1;

        // Handle encoding information
        let encoding = if words_read < header_size {
            const ENCODING_FIELD_LEN: u8 = 39;
            let encoding_len = self.reader.read_be::<u8>()?;
            words_read += 10;
            if encoding_len > 39 {
                return Err(DviousError::TfmParseError(format!(
                    "TFM header encoding specified to have [{}] bytes, which is more than the allowed 0..39",
                    encoding_len
                )));
            }

            let s = self.read_utf8_string(encoding_len as usize)?;
            self.reader.skip_bytes(ENCODING_FIELD_LEN - encoding_len)?;
            Option::Some(s)
        } else {
            Option::None
        };

        // Font identifier
        let font_identifier = if words_read < header_size {
            // The font identifier is saved in Pascal format,
            // the first byte indicates its length
            const FONT_ID_FIELD_LEN: u8 = 20;
            let font_id_len = self.reader.read_be::<u8>()?;
            if font_id_len > 19 {
                return Err(DviousError::TfmParseError(format!(
                    "TFM header font identifier specified to have [{}] bytes, which is more than the allowed 0..19",
                    font_id_len
                )));
            }
            let s = self.read_utf8_string(usize::from(font_id_len))?;
            self.reader.skip_bytes(FONT_ID_FIELD_LEN - font_id_len - 1)?;
            words_read += 5;
            Option::Some(s)
        } else {
            Option::None
        };

        // Face
        let face = if words_read < header_size {
            self.reader.read_be::<u8>()?;
            self.reader.read_be::<u16>()?;
            let face_byte = self.reader.read_be::<u8>()?;

            words_read += 1;
            Option::Some(face_byte)
        } else {
            Option::None
        };

        // Misc
        let mut misc = Vec::new();
        let misc_bytes = (header_size - words_read) * 4;

        for _ in 0..misc_bytes {
            let b = self.reader.read_be::<u8>()?;
            misc.push(b);
        }

        Ok(TfmMetricHeader {
            checksum,
            design_size,
            encoding,
            font_identifier,
            face,
            misc,
        })
    }

    fn read_char_info_table(&mut self, bc: u8, ec: u8) -> DviousResult<HashMap<u8, TfmCharInfo>> {
        let mut result = HashMap::new();

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

            let tag = self.read_char_info_table_tag(tag_value, remainder)?;

            let char_info_table = TfmCharInfo {
                character,
                width_index,
                height_index,
                depth_index,
                italic_index,
                tag,
            };
            result.insert(character, char_info_table);
        }

        Ok(result)
    }

    fn read_char_info_table_tag(
        &self,
        tag_value: u8,
        remainder: u8,
    ) -> DviousResult<TfmCharInfoTag> {
        let tag = match tag_value {
            0 => TfmCharInfoTag::None,
            1 => TfmCharInfoTag::Ligature(remainder),
            2 => TfmCharInfoTag::List(remainder),
            3 => TfmCharInfoTag::Extensible(remainder),
            _ => {
                return Err(DviousError::TfmParseError(format!(
                    "TFM character information specified invalid tag: {}",
                    tag_value
                )))
            }
        };
        Ok(tag)
    }

    fn read_lig_kern_table(&mut self, nl: u16) -> DviousResult<Vec<TfmLigatureCommand>> {
        let number_of_commands = nl as usize;
        let mut result = Vec::with_capacity(number_of_commands);

        for _ in 0..number_of_commands {
            let skip_byte = self.reader.read_be::<u8>()?;
            let next_char = self.reader.read_be::<u8>()?;
            let op_byte = self.reader.read_be::<u8>()?;
            let remainder = self.reader.read_be::<u8>()?;
            let cmd = TfmLigatureCommand {
                skip_byte,
                next_char,
                op_byte,
                remainder,
            };
            result.push(cmd);
        }
        Ok(result)
    }

    fn read_extension_table(&mut self, ne: u16) -> DviousResult<Vec<TfmExtensionRecipe>> {
        let n = ne as usize;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let top = self.reader.read_be::<u8>()?;
            let mid = self.reader.read_be::<u8>()?;
            let bot = self.reader.read_be::<u8>()?;
            let rep = self.reader.read_be::<u8>()?;
            let ext = TfmExtensionRecipe { top, mid, bot, rep };
            result.push(ext);
        }
        Ok(result)
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
    use crate::fonts::tfm::*;

    // Sanity checks

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_read_preamble_invalid_character_code_range() {
        let data = vec![
            0x00, 0x08,
            0x00, 0x00,
            0x00, 0x0E,
            0x00, 0x0D,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
        ];
        let mut tfm_reader = TfmMetricReader::new(data);

        if let DviousError::TfmParseError(error_message) = tfm_reader.read().err().unwrap() {
            assert_eq!(error_message, "The character code range [14]..[13] is illegal!");
        } else {
            panic!("Expected TfmParseError")
        }
    }

    // Header

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_read_header() {
        let data = vec![
            0xAA, 0xBB, 0xCC, 0xDD,

            0x00, 0xA0, 0x00, 0x00,

            0x04, 0x54, 0x65, 0x73, 0x74, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

            0x09, 0x48, 0x45, 0x4c, 0x56, 0x45, 0x54, 0x49,
            0x43, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x12,

            0xAA, 0xBB, 0xCC, 0xDD
        ];
        let number_of_bytes = data.len() as u16;
        let mut tfm_reader = TfmMetricReader::new(data);

        let header = tfm_reader.read_header(number_of_bytes / 4).unwrap();

        assert_eq!(
            header,
            TfmMetricHeader {
                checksum: 0xAABBCCDD,
                design_size: 10.0,
                encoding: Some("Test".to_string()),
                font_identifier: Some("HELVETICA".to_string()),
                face: Option::Some(0x12),
                misc: vec![0xAA, 0xBB, 0xCC, 0xDD]
            }
        );
    }

    // Char info

    #[test]
    fn test_read_charinfo() {
        let data = vec![0x42, 0xAB, 0b101010_10, 0xCD, 0x23, 0xCD, 0b010101_01, 0xEF];
        let mut tfm_reader = TfmMetricReader::new(data);

        let char_info_table = tfm_reader.read_char_info_table(0x60, 0x61).unwrap();

        assert_eq!(
            char_info_table.len(),
            2,
            "Expected only 2 char info entries!"
        );
        assert_eq!(
            char_info_table[&0x60],
            TfmCharInfo {
                character: 0x60,
                width_index: 0x42,
                height_index: 0xA * 16,
                depth_index: 0xB,
                italic_index: 42 * 4,
                tag: TfmCharInfoTag::List(0xCD),
            }
        );
        assert_eq!(
            char_info_table[&0x61],
            TfmCharInfo {
                character: 0x61,
                width_index: 0x23,
                height_index: 0xC * 16,
                depth_index: 0xD,
                italic_index: 21 * 4,
                tag: TfmCharInfoTag::Ligature(0xEF),
            }
        );
    }

    #[test]
    fn test_read_charinfo_tag_none() {
        let data = vec![];
        let tfm_reader = TfmMetricReader::new(data);

        let tag = tfm_reader.read_char_info_table_tag(0, 0x42).unwrap();

        assert_eq!(tag, TfmCharInfoTag::None);
    }

    #[test]
    fn test_read_charinfo_tag_ligature() {
        let data = vec![];
        let tfm_reader = TfmMetricReader::new(data);

        let tag = tfm_reader.read_char_info_table_tag(1, 0x42).unwrap();

        assert_eq!(tag, TfmCharInfoTag::Ligature(0x42));
    }

    #[test]
    fn test_read_charinfo_tag_list() {
        let data = vec![];
        let tfm_reader = TfmMetricReader::new(data);

        let tag = tfm_reader.read_char_info_table_tag(2, 0x42).unwrap();

        assert_eq!(tag, TfmCharInfoTag::List(0x42));
    }

    #[test]
    fn test_read_charinfo_tag_extensible() {
        let data = vec![];
        let tfm_reader = TfmMetricReader::new(data);

        let tag = tfm_reader.read_char_info_table_tag(3, 0x42).unwrap();

        assert_eq!(tag, TfmCharInfoTag::Extensible(0x42));
    }

    // Ligature/Kern table

    #[test]
    fn test_read_lig_kern_table() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
        let mut tfm_reader = TfmMetricReader::new(data);

        let lig_kern_table = tfm_reader.read_lig_kern_table(2).unwrap();

        assert_eq!(
            lig_kern_table,
            vec![
                TfmLigatureCommand {
                    skip_byte: 0xDE,
                    next_char: 0xAD,
                    op_byte: 0xBE,
                    remainder: 0xEF,
                },
                TfmLigatureCommand {
                    skip_byte: 0xCA,
                    next_char: 0xFE,
                    op_byte: 0xBA,
                    remainder: 0xBE,
                },
            ]
        );
    }

    // Extension table

    #[test]
    fn test_read_extension_table() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
        let mut tfm_reader = TfmMetricReader::new(data);

        let extension_table = tfm_reader.read_extension_table(2).unwrap();

        assert_eq!(
            extension_table,
            vec![
                TfmExtensionRecipe {
                    top: 0xDE,
                    mid: 0xAD,
                    bot: 0xBE,
                    rep: 0xEF,
                },
                TfmExtensionRecipe {
                    top: 0xCA,
                    mid: 0xFE,
                    bot: 0xBA,
                    rep: 0xBE,
                },
            ]
        );
    }

    // Fixword

    #[test]
    fn test_read_fixword_min() {
        let mut tfm_reader = TfmMetricReader::new(vec![0x80, 0x00, 0x00, 0x00]);

        let fixword = tfm_reader.read_fixword().unwrap();

        assert_eq!(fixword, -2048.0);
    }

    #[test]
    fn test_read_fixword_max() {
        let mut tfm_reader = TfmMetricReader::new(vec![0x7F, 0xFF, 0xFF, 0xFF]);

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
        let mut tfm_reader = TfmMetricReader::new(data);

        let fixword_table = tfm_reader.read_fixword_table(3).unwrap();

        assert_eq!(fixword_table, vec![-2048.0, 0.0, 0.6708984375]);
    }

    // Read string

    #[test]
    fn test_read_string() {
        let mut tfm_reader = TfmMetricReader::new(vec![0x54, 0x65, 0x73, 0x74]);

        let result = tfm_reader.read_utf8_string(4).unwrap();

        assert_eq!(result, "Test".to_string());
    }
}
