use crate::{Bit, UInt, B0, B1};

pub trait Unsigned {}
pub trait Mode {}
pub trait Formula {
    type Output: Unsigned + Formula;
}

impl Formula for B0 {
    type Output = B0;
}
impl Formula for B1 {
    type Output = B1;
}
impl<U: Unsigned, B: Bit> Formula for UInt<U, B> {
    type Output = UInt<U, B>;
}
