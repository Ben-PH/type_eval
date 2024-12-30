use core::marker::PhantomData;

use crate::{
    MemRep, NumExpr,
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

impl MemRep for _0 {
    const MU128: u128 = 0;
}
impl MemRep for _1 {
    const MU128: u128 = 1;
}

impl<Bs, Bt> MemRep for B<Bs, Bt>
where
    Bs: MemRep,
    Bt: MemRep,
{
    /// in-memory u128 representation
    const MU128: u128 = Bs::MU128 << 1 | Bt::MU128;
}

#[cfg(test)]
#[allow(clippy::used_underscore_items)]
mod test {
    use crate::{
        test_res::*,
        val_types::{B, _0, _1},
    };
    use crate::{num_vals::*, prelude::U4, MemRep};
    #[test]
    fn eval_trim() {
        const _0_0: () = _b0::<B<_0, _0>>();
        const _0_1: () = _b1::<B<_0, _1>>();
    }
    #[test]
    fn eval_mem_rep() {
        assert_eq!(0, U0::MU128);
        assert_eq!(1, U1::MU128);
        assert_eq!(2, U2::MU128);
        assert_eq!(3, U3::MU128);
        assert_eq!(4, U4::MU128);
        assert_eq!(5, U5::MU128);
        assert_eq!(6, U6::MU128);

        assert_eq!(0, U0::MUSIZE);
        assert_eq!(1, U1::MUSIZE);
        assert_eq!(2, U2::MUSIZE);
        assert_eq!(3, U3::MUSIZE);
        assert_eq!(4, U4::MUSIZE);
        assert_eq!(5, U5::MUSIZE);
        assert_eq!(6, U6::MUSIZE);

        assert_eq!(0, U0::MU64);
        assert_eq!(1, U1::MU64);
        assert_eq!(2, U2::MU64);
        assert_eq!(3, U3::MU64);
        assert_eq!(4, U4::MU64);
        assert_eq!(5, U5::MU64);
        assert_eq!(6, U6::MU64);

        assert_eq!(0, U0::MU32);
        assert_eq!(1, U1::MU32);
        assert_eq!(2, U2::MU32);
        assert_eq!(3, U3::MU32);
        assert_eq!(4, U4::MU32);
        assert_eq!(5, U5::MU32);
        assert_eq!(6, U6::MU32);

        assert_eq!(0, U0::MU16);
        assert_eq!(1, U1::MU16);
        assert_eq!(2, U2::MU16);
        assert_eq!(3, U3::MU16);
        assert_eq!(4, U4::MU16);
        assert_eq!(5, U5::MU16);
        assert_eq!(6, U6::MU16);

        assert_eq!(0, U0::MU8);
        assert_eq!(1, U1::MU8);
        assert_eq!(2, U2::MU8);
        assert_eq!(3, U3::MU8);
        assert_eq!(4, U4::MU8);
        assert_eq!(5, U5::MU8);
        assert_eq!(6, U6::MU8);
    }
}
