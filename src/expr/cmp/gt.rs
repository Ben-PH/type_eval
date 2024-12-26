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
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_gt() {
        const _0_GT_0: () = _f::<GT<U0, U0>>();
        const _1_GT_0: () = _t::<GT<U1, U0>>();
        const _1_GT_1: () = _f::<GT<U1, U1>>();
        const _2_GT_1: () = _t::<GT<U2, U1>>();
        const _1_GT_2: () = _f::<GT<U1, U2>>();
        const _3_GT_1: () = _t::<GT<U3, U1>>();
        const _4_GT_1: () = _t::<GT<U4, U1>>();
        // const _5_GT_6: () = _f::<GT<U5, U6>>();
        const _1_GT_3: () = _f::<GT<U1, U3>>();
        const _1_GT_4: () = _f::<GT<U1, U4>>();
        const _2_GT_2: () = _f::<GT<U2, U2>>();
        const _3_GT_3: () = _f::<GT<U3, U3>>();
        const _6_GT_1: () = _t::<GT<U6, U1>>();
        const _7_GT_1: () = _t::<GT<U7, U1>>();
    }
}
