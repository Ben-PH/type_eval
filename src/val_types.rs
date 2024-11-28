use core::marker::PhantomData;

use crate::Unsigned;



pub trait Bit {}
pub struct B0;
pub struct B1;
impl Bit for B0 {}
impl Bit for B1 {}
impl Unsigned for B0 {}
impl Unsigned for B1 {}

pub struct UInt<U: Unsigned, B: Bit> {
    _trailing_bit: PhantomData<B>,
    _leading_bits: PhantomData<U>,
}
trait NonZero: Unsigned {}
impl NonZero for B1 {}
impl<U: Unsigned, B: Bit> NonZero for UInt<U, B> {}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {}

