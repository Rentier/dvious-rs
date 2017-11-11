extern crate dvious;
use dvious::fonts::kpsewhich::*;

#[test]
fn test_get_path_to_pk() {
    let s = get_path_to_pk("cmr10");

    assert!(s.is_ok());
}
