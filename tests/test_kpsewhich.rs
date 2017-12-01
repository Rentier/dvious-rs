extern crate dvious;
use dvious::fonts::kpsewhich::*;

#[test]
fn test_get_path_to_pk_with_existing() {
    let s = get_path_to_pk("cmr10");

    assert!(s.is_ok(), "Expected Ok, was Err");
}

#[test]
fn test_get_path_to_pk_with_nonexisting() {
    let s = get_path_to_pk("nonexisting");

    assert!(s.is_err(), "Expected Err, was Ok");
}

#[test]
fn test_get_path_to_tfm_with_existing() {
    let s = get_path_to_tfm("cmr10");

    assert!(s.is_ok(), "Expected Ok, was Err");
}

#[test]
fn test_get_path_to_tfm_with_nonexisting() {
    let s = get_path_to_tfm("nonexisting");

    assert!(s.is_err(), "Expected Err, was Ok");
}
