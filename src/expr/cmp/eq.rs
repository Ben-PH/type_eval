use crate::{
    _inners::{_Base, _BitLit, _BitStrLit},
    ctrl_types::{False, True, AND, EQ},
    prelude::B,
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
impl<LBH, LBT, RBH, RBT> BoolExpr for EQ<B<LBH, LBT>, B<RBH, RBT>, _Base>
where
    LBH: NumExpr,
    LBT: NumExpr,
    RBH: _BitStrLit,
    RBT: _BitLit,
    EQ<LBT, RBT>: BoolExpr,
    EQ<LBH, RBH>: BoolExpr,
    AND<EQ<LBH, RBH>, EQ<LBT, RBT>>: BoolExpr,
{
    type Ret = BoolRet<AND<EQ<LBH, RBH>, EQ<LBT, RBT>>>;
}
