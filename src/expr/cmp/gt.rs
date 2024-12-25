use crate::{
    _inners::{_Base, _BitLit, _BitStrLit},
    ctrl_types::{False, True, GT},
    prelude::B as BitString,
    val_types::{_0, _1},
    BoolExpr, BoolRet, NumExpr,
};

impl<L, R> BoolExpr for GT<L, R>
where
    L: NumExpr,
    R: NumExpr,
    GT<L::Ret, R::Ret, _Base>: BoolExpr,
{
    type Ret = BoolRet<GT<L::Ret, R::Ret, _Base>>;
}
impl BoolExpr for GT<_1, _0, _Base> {
    type Ret = True;
}
impl BoolExpr for GT<_0, _1, _Base> {
    type Ret = False;
}
impl BoolExpr for GT<_0, _0, _Base> {
    type Ret = False;
}
impl BoolExpr for GT<_1, _1, _Base> {
    type Ret = False;
}

impl<Bs, B, R> BoolExpr for GT<BitString<Bs, B>, R, _Base>
where
    Bs: _BitStrLit,
    B: _BitLit,
    R: _BitLit,
{
    type Ret = True;
}

impl<Bs, B, L> BoolExpr for GT<L, BitString<Bs, B>, _Base>
where
    Bs: _BitStrLit,
    B: _BitLit,
    L: _BitLit,
{
    type Ret = False;
}
impl<LBs, LB, RBs, RB> BoolExpr for GT<BitString<LBs, LB>, BitString<RBs, RB>, _Base>
where
    LBs: _BitStrLit,
    LB: _BitLit,
    RBs: _BitStrLit,
    RB: _BitLit,
    GT<LB, RB>: BoolExpr,
{
    type Ret = BoolRet<GT<LB, RB>>;
}
