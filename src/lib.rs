#![no_std]
mod ctrl_types;
mod op_types;
mod val_types;
use ctrl_types::BoolVal;
use val_types::NumberVal;
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
#[cfg(test)]
mod test_res {
    use ctrl_types::{False, True};
    use val_types::{BitString, _0};

    use crate::val_types::_1;

    use super::*;
    pub(crate) const fn _b0<E: NumExpr<Ret = _0>>() {}
    pub(crate) const fn _b1<E: NumExpr<Ret = _1>>() {}
    pub(crate) const fn _b2<E: NumExpr<Ret = BitString<_1, _0>>>() {}
    pub(crate) const fn _b3<E: NumExpr<Ret = BitString<_1, _1>>>() {}
    pub(crate) const fn _b4<E: NumExpr<Ret = BitString<BitString<_1, _0>, _0>>>() {}
    pub(crate) const fn _b5<E: NumExpr<Ret = BitString<BitString<_1, _0>, _1>>>() {}
    pub(crate) const fn _b6<E: NumExpr<Ret = BitString<BitString<_1, _1>, _0>>>() {}
    pub(crate) const fn _b7<E: NumExpr<Ret = BitString<BitString<_1, _1>, _1>>>() {}
    pub(crate) const fn _b8<E: NumExpr<Ret = BitString<BitString<BitString<_1, _0>, _0>, _0>>>() {}
    pub(crate) const fn _b9<E: NumExpr<Ret = BitString<BitString<BitString<_1, _0>, _0>, _1>>>() {}

    pub(crate) const fn _t<E: BoolExpr<Ret = True>>() {}
    pub(crate) const fn _f<E: BoolExpr<Ret = False>>() {}
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
