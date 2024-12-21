#![no_std]
mod ctrl_types;
mod op_types;
mod val_types;
use ctrl_types::BoolVal;
use val_types::{BitString, NumberVal, _0, _1};
mod expr;

pub trait ExprMode {}
pub struct Recurse;
pub struct Base;
impl ExprMode for Recurse {}
impl ExprMode for Base {}

pub trait NumExpr {
    type Ret: NumberVal;
}
type NumRet<T> = <T as NumExpr>::Ret;

pub trait BoolExpr {
    type Ret: BoolVal;
}
type BoolRet<T> = <T as BoolExpr>::Ret;

// Some initial type aliases before refining the generator
pub type U0 = _0;
pub type U1 = _1;
pub type U2 = BitString<_1, _0>;
pub type U3 = BitString<_1, _1>;
pub type U4 = BitString<U2, _0>;
pub type U5 = BitString<U2, _1>;
pub type U6 = BitString<U3, _0>;
pub type U7 = BitString<U3, _1>;
pub type U8 = BitString<U4, _0>;
pub type U9 = BitString<U4, _1>;
pub type U10 = BitString<U5, _0>;
pub type U11 = BitString<U5, _1>;
pub type U12 = BitString<U6, _0>;
pub type U13 = BitString<U6, _1>;
pub type U14 = BitString<U7, _0>;
pub type U15 = BitString<U7, _1>;
pub type U16 = BitString<U8, _0>;
pub type U17 = BitString<U8, _1>;
pub type U18 = BitString<U9, _0>;
pub type U19 = BitString<U9, _1>;

#[cfg(test)]
mod test_res {
    use ctrl_types::{False, True};
    use val_types::{BitString, _0};

    use crate::val_types::_1;

    use super::*;
    pub(crate) const fn _b0<E: NumExpr<Ret = _0>>() {}
    pub(crate) const fn _b1<E: NumExpr<Ret = _1>>() {}
    pub(crate) const fn _b2<E: NumExpr<Ret = U2>>() {}
    pub(crate) const fn _b3<E: NumExpr<Ret = U3>>() {}
    pub(crate) const fn _b4<E: NumExpr<Ret = U4>>() {}
    pub(crate) const fn _b5<E: NumExpr<Ret = U5>>() {}
    pub(crate) const fn _b6<E: NumExpr<Ret = U6>>() {}
    pub(crate) const fn _b7<E: NumExpr<Ret = U7>>() {}
    pub(crate) const fn _b8<E: NumExpr<Ret = U8>>() {}
    pub(crate) const fn _b9<E: NumExpr<Ret = U9>>() {}

    pub(crate) const fn _t<E: BoolExpr<Ret = True>>() {}
    pub(crate) const fn _f<E: BoolExpr<Ret = False>>() {}
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
