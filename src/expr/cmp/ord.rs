use core::marker::PhantomData;

use crate::{
    ctrl_types::{Eq, Gr8r, Less, OrdVal, ORD},
    val_types::{NumberVal, B, _0, _1},
    NumExpr, NumRet, OrdExpr, OrdRet,
    _inners::{_Base, _BitLit},
};

impl<Lhs, Rhs> OrdExpr for ORD<Lhs, Rhs>
where
    Lhs: NumExpr,
    Rhs: NumExpr,
    NumRet<Lhs>: NumberVal,
    NumRet<Rhs>: NumberVal,
    ORD<NumRet<Lhs>, NumRet<Rhs>, _Base>: OrdExpr,
{
    type Ret = OrdRet<ORD<NumRet<Lhs>, NumRet<Rhs>, _Base>>;
}

impl OrdExpr for ORD<_0, _0, _Base> {
    type Ret = Eq;
}
impl OrdExpr for ORD<_0, _1, _Base> {
    type Ret = Less;
}
impl OrdExpr for ORD<_1, _0, _Base> {
    type Ret = Gr8r;
}
impl OrdExpr for ORD<_1, _1, _Base> {
    type Ret = Eq;
}
impl<LBs, LB, RB> OrdExpr for ORD<B<LBs, LB>, RB, _Base>
where
    RB: _BitLit,
{
    type Ret = Gr8r;
}
impl<LB, RBs, RB> OrdExpr for ORD<LB, B<RBs, RB>, _Base>
where
    LB: _BitLit,
{
    type Ret = Less;
}
impl<LBs, LB, RBs, RB> OrdExpr for ORD<B<LBs, LB>, B<RBs, RB>, _Base>
where
    BitwiseORD<Eq, B<LBs, LB>, B<RBs, RB>>: OrdExpr,
{
    type Ret = OrdRet<BitwiseORD<Eq, B<LBs, LB>, B<RBs, RB>>>;
}

pub struct BitwiseORD<C: OrdVal, L, R> {
    _cache: PhantomData<C>,
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
}
impl<C, RBs, RB> OrdExpr for BitwiseORD<C, _1, B<RBs, RB>>
where
    C: OrdVal,
{
    type Ret = Less;
}
impl<C> OrdExpr for BitwiseORD<C, _1, _1>
where
    C: OrdVal,
{
    type Ret = C;
}
impl<C, LBs, LB> OrdExpr for BitwiseORD<C, B<LBs, LB>, _1>
where
    C: OrdVal,
{
    type Ret = Gr8r;
}

impl<C, LBs, LB> OrdExpr for BitwiseORD<C, B<LBs, LB>, _0>
where
    C: OrdVal,
{
    type Ret = Gr8r;
}

impl<C, LBs, RBs> OrdExpr for BitwiseORD<C, B<LBs, _0>, B<RBs, _0>>
where
    C: OrdVal,
    BitwiseORD<C, LBs, RBs>: OrdExpr,
{
    type Ret = OrdRet<BitwiseORD<C, LBs, RBs>>;
}
impl<C, LBs, RBs> OrdExpr for BitwiseORD<C, B<LBs, _1>, B<RBs, _1>>
where
    C: OrdVal,
    BitwiseORD<C, LBs, RBs>: OrdExpr,
{
    type Ret = OrdRet<BitwiseORD<C, LBs, RBs>>;
}
impl<C, LBs, RBs> OrdExpr for BitwiseORD<C, B<LBs, _0>, B<RBs, _1>>
where
    C: OrdVal,
    BitwiseORD<Less, LBs, RBs>: OrdExpr,
{
    type Ret = OrdRet<BitwiseORD<Less, LBs, RBs>>;
}
impl<C, LBs, RBs> OrdExpr for BitwiseORD<C, B<LBs, _1>, B<RBs, _0>>
where
    C: OrdVal,
    BitwiseORD<Gr8r, LBs, RBs>: OrdExpr,
{
    type Ret = OrdRet<BitwiseORD<Gr8r, LBs, RBs>>;
}

#[cfg(test)]
#[allow(clippy::used_underscore_items)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U5, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_ord() {
        const _0_ORD_0: () = _eq::<ORD<_0, _0>>();
        const _1_ORD_0: () = _gt::<ORD<_1, _0>>();
        const _1_ORD_1: () = _eq::<ORD<U1, U1>>();
        const _2_ORD_0: () = _gt::<ORD<U2, U0>>();
        const _0_ORD_2: () = _lt::<ORD<U0, U2>>();
        const _2_ORD_1: () = _gt::<ORD<U2, U1>>();
        const _1_ORD_2: () = _lt::<ORD<U1, U2>>();
        const _3_ORD_1: () = _gt::<ORD<U3, U1>>();
        const _4_ORD_1: () = _gt::<ORD<U4, U1>>();
        const _5_ORD_6: () = _lt::<ORD<U5, U6>>();
        const _6_ORD_5: () = _gt::<ORD<U6, U5>>();
        const _1_ORD_3: () = _lt::<ORD<U1, U3>>();
        const _1_ORD_4: () = _lt::<ORD<U1, U4>>();
        const _2_ORD_2: () = _eq::<ORD<U2, U2>>();
        const _3_ORD_3: () = _eq::<ORD<U3, U3>>();
        const _6_ORD_1: () = _gt::<ORD<U6, U1>>();
        const _7_ORD_1: () = _gt::<ORD<U7, U1>>();
    }
}
