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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_lt() {
        const _0_LT_0: () = _f::<LT<U0, U0>>();
        const _1_LT_0: () = _f::<LT<U1, U0>>();
        const _1_LT_1: () = _f::<LT<U1, U1>>();
        const _2_LT_1: () = _f::<LT<U2, U1>>();
        const _1_LT_2: () = _t::<LT<U1, U2>>();
        const _3_LT_1: () = _f::<LT<U3, U1>>();
        const _4_LT_1: () = _f::<LT<U4, U1>>();
        // const _5_LT_6: () = _t::<LT<U5, U6>>();
        const _1_LT_3: () = _t::<LT<U1, U3>>();
        const _1_LT_4: () = _t::<LT<U1, U4>>();
        const _2_LT_2: () = _f::<LT<U2, U2>>();
        const _3_LT_3: () = _f::<LT<U3, U3>>();
        const _6_LT_1: () = _f::<LT<U6, U1>>();
        const _7_LT_1: () = _f::<LT<U7, U1>>();
    }
}
