use crate::{
    ctrl_types::{AND, EQ, GT, GTE, LT, LTE},
    Base, BoolExpr, BoolRet, NumExpr,
};

impl<L, R> BoolExpr for GTE<L, R>
where
    L: NumExpr,
    R: NumExpr,
    GT<L::Ret, R::Ret, Base>: BoolExpr,
    EQ<L::Ret, R::Ret, Base>: BoolExpr,
    AND<GT<L::Ret, R::Ret, Base>, EQ<L::Ret, R::Ret, Base>>: BoolExpr,
{
    type Ret = BoolRet<AND<GT<L::Ret, R::Ret, Base>, EQ<L::Ret, R::Ret, Base>>>;
}
impl<L, R> BoolExpr for LTE<L, R>
where
    L: NumExpr,
    R: NumExpr,
    LT<L::Ret, R::Ret, Base>: BoolExpr,
    EQ<L::Ret, R::Ret, Base>: BoolExpr,
    AND<LT<L::Ret, R::Ret, Base>, EQ<L::Ret, R::Ret, Base>>: BoolExpr,
{
    type Ret = BoolRet<AND<LT<L::Ret, R::Ret, Base>, EQ<L::Ret, R::Ret, Base>>>;
}
