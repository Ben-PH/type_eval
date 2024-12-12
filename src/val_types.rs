use core::marker::PhantomData;

use crate::Expr;

pub trait Number {}
pub struct _0;
pub struct _1;
pub struct BitString<Bs : BitStrLit, B : BitLit> {
    _bits: PhantomData<Bs>,
    _last_bit: PhantomData<B>,
    // _m: PhantomData<M>,
}

impl Number for _0 {}
impl Number for _1 {}
impl<Bs: BitStrLit, B: BitLit> Number for BitString<Bs, B> {}

pub trait BitLit {}

impl BitLit for _0 {}
impl BitLit for _1 {}

pub trait BitStrLit {}
impl BitStrLit for _1 {}
impl<Bs: BitStrLit, B: BitLit> BitStrLit for BitString<Bs, B> {}

impl Expr for _0 {
    type Output = _0;
}
impl Expr for _1 {
    type Output = _1;
}
