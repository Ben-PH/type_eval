use crate::{
    ctrl_types::{False, True, AND, EQ},
    val_types::{BitLit, BitStrLit, BitString, _0, _1},
    Base, BoolExpr, BoolRet, NumExpr,
};
impl<L, R> BoolExpr for EQ<L, R>
where
    L: NumExpr,
    R: NumExpr,
    EQ<L::Ret, R::Ret, Base>: BoolExpr,
{
    type Ret = BoolRet<EQ<L::Ret, R::Ret, Base>>;
}
impl BoolExpr for EQ<_1, _0, Base> {
    type Ret = False;
}
impl BoolExpr for EQ<_0, _1, Base> {
    type Ret = False;
}
impl BoolExpr for EQ<_0, _0, Base> {
    type Ret = True;
}
impl BoolExpr for EQ<_1, _1, Base> {
    type Ret = True;
}
impl<LBs, LB, RBs, RB> BoolExpr for EQ<BitString<LBs, LB>, BitString<RBs, RB>, Base>
where
    LBs: NumExpr,
    LB: NumExpr,
    RBs: BitStrLit,
    RB: BitLit,
    EQ<LB, RB>: BoolExpr,
    EQ<LBs, RBs>: BoolExpr,
    AND<EQ<LBs, RBs>, EQ<LB, RB>>: BoolExpr,
{
    type Ret = BoolRet<AND<EQ<LBs, RBs>, EQ<LB, RB>>>;
}
