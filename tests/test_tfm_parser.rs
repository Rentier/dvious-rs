extern crate dvious;
use dvious::fonts::kpsewhich::get_path_to_tfm;
use dvious::fonts::tfm::*;

#[test]
fn test_parse_tfm_file() {
    let path = get_path_to_tfm("cmr10").unwrap();
    let tfm = read_tfm_from_file(path).unwrap();

    assert_eq!(tfm.header.checksum, 1_274_110_073);
    assert_eq!(tfm.header.design_size, 10.);
    assert_eq!(tfm.header.encoding, Some(String::from("TeX text")));
    assert_eq!(tfm.header.font_identifier, Some(String::from("CMR")));
}
