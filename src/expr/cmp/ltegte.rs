use crate::{
    _inners::_Base,
    ctrl_types::{AND, EQ, GT, GTE, LT, LTE},
    BoolExpr, BoolRet, NumExpr,
};

impl<L, R> BoolExpr for GTE<L, R>
where
    L: NumExpr,
    R: NumExpr,
    GT<L::Ret, R::Ret, _Base>: BoolExpr,
    EQ<L::Ret, R::Ret, _Base>: BoolExpr,
    AND<GT<L::Ret, R::Ret, _Base>, EQ<L::Ret, R::Ret, _Base>>: BoolExpr,
{
    type Ret = BoolRet<AND<GT<L::Ret, R::Ret, _Base>, EQ<L::Ret, R::Ret, _Base>>>;
}
impl<L, R> BoolExpr for LTE<L, R>
where
    L: NumExpr,
    R: NumExpr,
    LT<L::Ret, R::Ret, _Base>: BoolExpr,
    EQ<L::Ret, R::Ret, _Base>: BoolExpr,
    AND<LT<L::Ret, R::Ret, _Base>, EQ<L::Ret, R::Ret, _Base>>: BoolExpr,
{
    type Ret = BoolRet<AND<LT<L::Ret, R::Ret, _Base>, EQ<L::Ret, R::Ret, _Base>>>;
}
