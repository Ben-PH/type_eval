use crate::{
    _inners::{_Base, _BitLit, _BitStrLit},
    ctrl_types::{False, True, AND, EQ},
    prelude::B as BitString,
    val_types::{_0, _1},
    BoolExpr, BoolRet, NumExpr,
};
impl<L, R> BoolExpr for EQ<L, R>
where
    L: NumExpr,
    R: NumExpr,
    EQ<L::Ret, R::Ret, _Base>: BoolExpr,
{
    type Ret = BoolRet<EQ<L::Ret, R::Ret, _Base>>;
}
impl BoolExpr for EQ<_1, _0, _Base> {
    type Ret = False;
}
impl BoolExpr for EQ<_0, _1, _Base> {
    type Ret = False;
}
impl BoolExpr for EQ<_0, _0, _Base> {
    type Ret = True;
}
impl BoolExpr for EQ<_1, _1, _Base> {
    type Ret = True;
}
impl<LBs, LB, RBs, RB> BoolExpr for EQ<BitString<LBs, LB>, BitString<RBs, RB>, _Base>
where
    LBs: NumExpr,
    LB: NumExpr,
    RBs: _BitStrLit,
    RB: _BitLit,
    EQ<LB, RB>: BoolExpr,
    EQ<LBs, RBs>: BoolExpr,
    AND<EQ<LBs, RBs>, EQ<LB, RB>>: BoolExpr,
{
    type Ret = BoolRet<AND<EQ<LBs, RBs>, EQ<LB, RB>>>;
}
