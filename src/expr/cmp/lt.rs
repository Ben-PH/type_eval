use crate::{
    ctrl_types::{False, True, LT},
    val_types::{BitLit, BitStrLit, BitString, _0, _1},
    Base, BoolExpr, BoolRet, NumExpr,
};

impl<L, R> BoolExpr for LT<L, R>
where
    L: NumExpr,
    R: NumExpr,
    LT<L::Ret, R::Ret, Base>: BoolExpr,
{
    type Ret = BoolRet<LT<L::Ret, R::Ret, Base>>;
}
impl BoolExpr for LT<_1, _0, Base> {
    type Ret = False;
}
impl BoolExpr for LT<_0, _1, Base> {
    type Ret = True;
}
impl BoolExpr for LT<_0, _0, Base> {
    type Ret = False;
}
impl BoolExpr for LT<_1, _1, Base> {
    type Ret = False;
}

impl<Bs, B, R> BoolExpr for LT<BitString<Bs, B>, R, Base>
where
    Bs: BitStrLit,
    B: BitLit,
    R: BitLit,
{
    type Ret = False;
}

impl<Bs, B, L> BoolExpr for LT<L, BitString<Bs, B>, Base>
where
    Bs: BitStrLit,
    B: BitLit,
    L: BitLit,
{
    type Ret = True;
}
impl<LBs, LB, RBs, RB> BoolExpr for LT<BitString<LBs, LB>, BitString<RBs, RB>, Base>
where
    LBs: BitStrLit,
    LB: BitLit,
    RBs: BitStrLit,
    RB: BitLit,
    LT<LB, RB>: BoolExpr,
{
    type Ret = BoolRet<LT<LB, RB>>;
}
