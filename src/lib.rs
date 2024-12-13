mod op_types;
mod val_types;
use val_types::Number;
mod expr;

pub trait ExprMode {}
pub struct Recurse;
pub struct Base;
impl ExprMode for Recurse {}
impl ExprMode for Base {}

pub trait Expr {
    type Ret: Number;
}
type ExpRet<T> = <T as Expr>::Ret;

#[cfg(test)]
mod test_res {
    use val_types::{BitString, _0};

    use crate::val_types::_1;

    use super::*;
    pub(crate) const fn _b0<E: Expr<Ret = _0>>() {}
    pub(crate) const fn _b1<E: Expr<Ret = _1>>() {}
    pub(crate) const fn _b2<E: Expr<Ret = BitString<_1, _0>>>() {}
    pub(crate) const fn _b3<E: Expr<Ret = BitString<_1, _1>>>() {}
    pub(crate) const fn _b4<E: Expr<Ret = BitString<BitString<_1, _0>, _0>>>() {}
    pub(crate) const fn _b5<E: Expr<Ret = BitString<BitString<_1, _0>, _1>>>() {}
    pub(crate) const fn _b6<E: Expr<Ret = BitString<BitString<_1, _1>, _0>>>() {}
    pub(crate) const fn _b8<E: Expr<Ret = BitString<BitString<BitString<_1, _0>, _0>, _0>>>() {}
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
