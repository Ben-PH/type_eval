use core::marker::PhantomData;

use crate::{
    _inners::_Base,
    ctrl_types::{False, True, GT},
    prelude::BoolVal,
    val_types::{B, _0, _1},
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
impl<LBs, LB, RBs, RB> BoolExpr for GT<B<LBs, LB>, B<RBs, RB>, _Base>
where
    GT<LB, RB, _Base>: BoolExpr,
    _BitwiseGT<BoolRet<GT<LB, RB, _Base>>, LBs, RBs>: BoolExpr,
{
    type Ret = BoolRet<_BitwiseGT<BoolRet<GT<LB, RB, _Base>>, LBs, RBs>>;
}
impl<RBs, RB> BoolExpr for GT<_0, B<RBs, RB>, _Base> {
    type Ret = False;
}
impl<RBs, RB> BoolExpr for GT<_1, B<RBs, RB>, _Base> {
    type Ret = False;
}
impl<LBs, LB> BoolExpr for GT<B<LBs, LB>, _1, _Base> {
    type Ret = True;
}
impl<LBs, LB> BoolExpr for GT<B<LBs, LB>, _0, _Base> {
    type Ret = True;
}

pub struct _BitwiseGT<C: BoolVal, L, R> {
    _cache: PhantomData<C>,
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
}
impl<C, RBs, RB> BoolExpr for _BitwiseGT<C, _1, B<RBs, RB>>
where
    C: BoolVal,
{
    type Ret = False;
}
impl<C> BoolExpr for _BitwiseGT<C, _1, _1>
where
    C: BoolVal,
{
    type Ret = C;
}
impl<C, LBs, LB> BoolExpr for _BitwiseGT<C, B<LBs, LB>, _1>
where
    C: BoolVal,
{
    type Ret = True;
}

impl<C, LBs, LB> BoolExpr for _BitwiseGT<C, B<LBs, LB>, _0>
where
    C: BoolVal,
{
    type Ret = True;
}

impl<C, LBs, RBs> BoolExpr for _BitwiseGT<C, B<LBs, _0>, B<RBs, _0>>
where
    C: BoolVal,
    _BitwiseGT<C, LBs, RBs>: BoolExpr,
{
    type Ret = BoolRet<_BitwiseGT<C, LBs, RBs>>;
}
impl<C, LBs, RBs> BoolExpr for _BitwiseGT<C, B<LBs, _1>, B<RBs, _1>>
where
    C: BoolVal,
    _BitwiseGT<C, LBs, RBs>: BoolExpr,
{
    type Ret = BoolRet<_BitwiseGT<C, LBs, RBs>>;
}
impl<C, LBs, RBs> BoolExpr for _BitwiseGT<C, B<LBs, _0>, B<RBs, _1>>
where
    C: BoolVal,
    _BitwiseGT<False, LBs, RBs>: BoolExpr,
{
    type Ret = BoolRet<_BitwiseGT<False, LBs, RBs>>;
}
impl<C, LBs, RBs> BoolExpr for _BitwiseGT<C, B<LBs, _1>, B<RBs, _0>>
where
    C: BoolVal,
    _BitwiseGT<True, LBs, RBs>: BoolExpr,
{
    type Ret = BoolRet<_BitwiseGT<True, LBs, RBs>>;
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U5, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_gt() {
        const _0_GT_0: () = _f::<GT<U0, U0>>();
        const _1_GT_0: () = _t::<GT<U1, U0>>();
        const _1_GT_1: () = _f::<GT<U1, U1>>();
        const _2_GT_1: () = _t::<GT<U2, U1>>();
        const _2_GT_0: () = _t::<GT<U2, U0>>();
        const _2_GT_4: () = _f::<GT<U2, U4>>();
        const _1_GT_2: () = _f::<GT<U1, U2>>();
        const _3_GT_1: () = _t::<GT<U3, U1>>();
        const _4_GT_1: () = _t::<GT<U4, U1>>();
        const _4_GT_2: () = _t::<GT<U4, U2>>();
        const _5_GT_6: () = _f::<GT<U5, U6>>();
        const _1_GT_3: () = _f::<GT<U1, U3>>();
        const _1_GT_4: () = _f::<GT<U1, U4>>();
        const _2_GT_2: () = _f::<GT<U2, U2>>();
        const _3_GT_3: () = _f::<GT<U3, U3>>();
        const _6_GT_1: () = _t::<GT<U6, U1>>();
        const _7_GT_1: () = _t::<GT<U7, U1>>();
    }
}
