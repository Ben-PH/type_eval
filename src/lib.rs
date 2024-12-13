mod op_types;
mod val_types;
use val_types::Number;
mod expr;

pub trait Mode {}
pub struct Ast;
pub struct Eval;
impl Mode for Ast {}
impl Mode for Eval {}

pub trait Expr {
    type Output: Number;
}
type ExprOut<T> = <T as Expr>::Output;


#[cfg(test)]
mod test_res {
    use val_types::{BitString, _0};

    use crate::val_types::_1;

    use super::*;
    pub(crate) const fn _b0<E: Expr<Output = _0>>() {}
    pub(crate) const fn _b1<E: Expr<Output = _1>>() {}
    pub(crate) const fn _b2<E: Expr<Output = BitString<_1, _0>>>() {}
    pub(crate) const fn _b3<E: Expr<Output = BitString<_1, _1>>>() {}
    pub(crate) const fn _b4<E: Expr<Output = BitString<BitString<_1, _0>, _0>>>() {}
    pub(crate) const fn _b5<E: Expr<Output = BitString<BitString<_1, _0>, _1>>>() {}
    pub(crate) const fn _b6<E: Expr<Output = BitString<BitString<_1, _1>, _0>>>() {}
    pub(crate) const fn _b8<E: Expr<Output = BitString<BitString<BitString<_1, _0>, _0>, _0>>>() {}
    #[test]
    fn eval_add() {
        const _0_0: () = _b0::<BitString<_0, _0>>();
        const _0_1: () = _b1::<BitString<_0, _1>>();
    }
}
