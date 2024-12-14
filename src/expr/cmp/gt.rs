use crate::{
    ctrl_types::{False, True, GT},
    val_types::{BitLit, BitStrLit, BitString, _0, _1},
    Base, BoolExpr, BoolRet, NumExpr,
};

impl<L, R> BoolExpr for GT<L, R>
where
    L: NumExpr,
    R: NumExpr,
    GT<L::Ret, R::Ret, Base>: BoolExpr,
{
    type Ret = BoolRet<GT<L::Ret, R::Ret, Base>>;
}
impl BoolExpr for GT<_1, _0, Base> {
    type Ret = True;
}
impl BoolExpr for GT<_0, _1, Base> {
    type Ret = False;
}
impl BoolExpr for GT<_0, _0, Base> {
    type Ret = False;
}
impl BoolExpr for GT<_1, _1, Base> {
    type Ret = False;
}

impl<Bs, B, R> BoolExpr for GT<BitString<Bs, B>, R, Base>
where
    Bs: BitStrLit,
    B: BitLit,
    R: BitLit,
{
    type Ret = True;
}

impl<Bs, B, L> BoolExpr for GT<L, BitString<Bs, B>, Base>
where
    Bs: BitStrLit,
    B: BitLit,
    L: BitLit,
{
    type Ret = False;
}
impl<LBs, LB, RBs, RB> BoolExpr for GT<BitString<LBs, LB>, BitString<RBs, RB>, Base>
where
    LBs: BitStrLit,
    LB: BitLit,
    RBs: BitStrLit,
    RB: BitLit,
    GT<LB, RB>: BoolExpr,
{
    type Ret = BoolRet<GT<LB, RB>>;
}
