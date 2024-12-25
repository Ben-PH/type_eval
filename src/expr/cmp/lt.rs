use crate::{
    _inners::{_Base, _BitLit, _BitStrLit},
    ctrl_types::{False, True, LT},
    prelude::B as BitString,
    val_types::{_0, _1},
    BoolExpr, BoolRet, NumExpr,
};

impl<L, R> BoolExpr for LT<L, R>
where
    L: NumExpr,
    R: NumExpr,
    LT<L::Ret, R::Ret, _Base>: BoolExpr,
{
    type Ret = BoolRet<LT<L::Ret, R::Ret, _Base>>;
}
impl BoolExpr for LT<_1, _0, _Base> {
    type Ret = False;
}
impl BoolExpr for LT<_0, _1, _Base> {
    type Ret = True;
}
impl BoolExpr for LT<_0, _0, _Base> {
    type Ret = False;
}
impl BoolExpr for LT<_1, _1, _Base> {
    type Ret = False;
}

impl<Bs, B, R> BoolExpr for LT<BitString<Bs, B>, R, _Base>
where
    Bs: _BitStrLit,
    B: _BitLit,
    R: _BitLit,
{
    type Ret = False;
}

impl<Bs, B, L> BoolExpr for LT<L, BitString<Bs, B>, _Base>
where
    Bs: _BitStrLit,
    B: _BitLit,
    L: _BitLit,
{
    type Ret = True;
}
impl<LBs, LB, RBs, RB> BoolExpr for LT<BitString<LBs, LB>, BitString<RBs, RB>, _Base>
where
    LBs: _BitStrLit,
    LB: _BitLit,
    RBs: _BitStrLit,
    RB: _BitLit,
    LT<LB, RB>: BoolExpr,
{
    type Ret = BoolRet<LT<LB, RB>>;
}
