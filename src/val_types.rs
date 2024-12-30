use core::marker::PhantomData;

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
impl<Bs, Bt> NumberVal for B<Bs, Bt>
where
    Bs: _BitStrLit,
    Bt: _BitLit,
{
}

/// Trims 0-leading bitstrings semi-automagically
impl<Bt> NumExpr for B<_0, Bt>
where
    Bt: NumberVal,
{
    type Ret = Bt;
}

impl<Bs, Bt> NumExpr for B<Bs, Bt>
where
    Bs: _BitStrLit,
    Bt: _BitLit,
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
#[allow(clippy::used_underscore_items)]
mod test {
    use crate::{
        test_res::*,
        val_types::{B, _0, _1},
    };
    #[test]
    fn eval_trim() {
        const _0_0: () = _b0::<B<_0, _0>>();
        const _0_1: () = _b1::<B<_0, _1>>();
    }
}
