use core::marker::PhantomData;

use crate::{Formula, Unsigned};

pub trait Bit {}
pub struct B0;
pub struct B1;
impl Bit for B0 {}
impl Bit for B1 {}
impl Unsigned for B0 {}
impl Unsigned for B1 {}
impl Formula for B0 {
    type Output = Self;
}
impl Formula for B1 {
    type Output = Self;
}

pub struct UInt<U, B> {
    _trailing_bit: PhantomData<B>,
    _leading_bits: PhantomData<U>,
}
impl<U, B> Formula for UInt<U, B> {
    type Output = Self;
}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {}
