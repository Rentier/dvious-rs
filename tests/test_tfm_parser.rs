extern crate dvious;
use dvious::fonts::kpsewhich::get_path_to_tfm;
use dvious::fonts::tfm::*;

#[test]
fn test_parse_tfm_file() {
    let path = get_path_to_tfm("cmr10").unwrap();
    let tfm = read_tfm_from_file(path).unwrap();
}
