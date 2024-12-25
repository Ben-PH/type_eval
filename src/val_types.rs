use core::marker::PhantomData;

use self::B as BitString;
use crate::{
    NumExpr,
    _inners::{_BitLit, _BitStrLit, _ExprMode, _Recurse},
};

pub trait NumberVal {}

/// Literal representation of the 0-bit
pub struct _0;
/// Literal representation of the 1-bit
pub struct _1;
/// Literal representation of a bit-string
pub struct B<Bs, B, M: _ExprMode = _Recurse> {
    _bits: PhantomData<Bs>,
    _last_bit: PhantomData<B>,
    _m: PhantomData<M>,
}

/// A [`NumExpr`] can output type-expressed `0`
impl NumberVal for _0 {}
/// A [`NumExpr`] can output type-expressed `1`
impl NumberVal for _1 {}
/// A [`NumExpr`] can output type-expressed `0bxxxx`
impl<Bs, B> NumberVal for BitString<Bs, B>
where
    Bs: _BitStrLit,
    B: _BitLit,
{
}

/// Trims 0-leading bitstrings semi-automagically
impl<B> NumExpr for BitString<_0, B>
where
    B: NumberVal,
{
    type Ret = B;
}

impl<Lhs, B> NumExpr for BitString<Lhs, B>
where
    Lhs: _BitStrLit,
    B: _BitLit,
{
    type Ret = Self;
}

impl NumExpr for _0 {
    type Ret = Self;
}
impl NumExpr for _1 {
    type Ret = Self;
}

#[cfg(test)]
mod test {
    use crate::{
        test_res::*,
        val_types::{BitString, _0, _1},
    };
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
