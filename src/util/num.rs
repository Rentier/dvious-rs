// Fixword

pub type Fixword = f64;

// u24

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub struct u24(u32);

impl From<u24> for u32 {
    fn from(num: u24) -> Self {
        num.0
    }
}

impl From<u32> for u24 {
    fn from(num: u32) -> Self {
        u24(num)
    }
}

impl From<u24> for i32 {
    fn from(num: u24) -> Self {
        num.0 as i32
    }
}

impl From<i32> for u24 {
    fn from(num: i32) -> Self {
        u24(num as u32)
    }
}

// i24

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub struct i24(i32);

impl From<i24> for i32 {
    fn from(num: i24) -> Self {
        num.0
    }
}

impl From<i32> for i24 {
    fn from(num: i32) -> Self {
        i24(num)
    }
}
