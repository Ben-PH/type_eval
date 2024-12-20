use core::marker::PhantomData;

use crate::{Ast, Formula, Mode, Unsigned};

pub trait Bit {}
pub struct B0;
pub struct B1;
impl Bit for B0 {}
impl Bit for B1 {}
impl Unsigned for B0 {}
impl Unsigned for B1 {}
impl Formula for B0 {
    type FOutput = Self;
}
impl Formula for B1 {
    type FOutput = Self;
}

pub struct UInt<U, B, M: Mode = Ast> {
    _trailing_bit: PhantomData<B>,
    _leading_bits: PhantomData<U>,
    _mode: PhantomData<M>,
}
impl<U, B> Formula for UInt<U, B> {
    type FOutput = Self;
}

impl<U: Unsigned, B: Bit> Unsigned for UInt<U, B> {}
