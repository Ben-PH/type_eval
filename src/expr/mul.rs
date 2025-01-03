use crate::{
    _inners::{_Base, _BitLit, _BitStrLit},
    num_vals::{U0, U1},
    op_types::{AddExp, MulExp},
    val_types::{NumberVal, B, _0, _1},
    NumExpr, NumRet,
};

impl<L, R> NumExpr for MulExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    MulExp<L::Ret, R::Ret, _Base>: NumExpr,
{
    type Ret = NumRet<MulExp<L::Ret, R::Ret, _Base>>;
}
// -------------
// Hard codod base cases.
// Past attempts to coallesce these impls resulted in overlapping-impl headaches
// -------------

impl NumExpr for MulExp<U0, U1, _Base> {
    type Ret = U0;
}
impl NumExpr for MulExp<U1, U0, _Base> {
    type Ret = U0;
}
impl NumExpr for MulExp<U0, U0, _Base> {
    type Ret = U0;
}
impl NumExpr for MulExp<U1, U1, _Base> {
    type Ret = U1;
}
impl<Bs, Bt> NumExpr for MulExp<U1, B<Bs, Bt>, _Base>
where
    Bs: NumExpr,
    Bt: _BitLit,
    B<Bs, Bt>: NumberVal,
{
    type Ret = B<Bs, Bt>;
}

// (LB, U1) * Val == ((LB * Val), U0) + Val
impl<LB, Val> NumExpr for MulExp<B<LB, _1>, Val, _Base>
where
    LB: _BitStrLit,
    MulExp<LB, Val>: NumExpr,
    AddExp<B<NumRet<MulExp<LB, Val>>, _0>, Val>: NumExpr,
{
    type Ret = NumRet<AddExp<B<NumRet<MulExp<LB, Val>>, _0>, Val>>;
}
// (LB, U0) * Val == ((LB * Val), U0)
impl<LB, Val> NumExpr for MulExp<B<LB, _0>, Val, _Base>
where
    LB: _BitStrLit,
    MulExp<LB, Val>: NumExpr,
    NumRet<MulExp<LB, Val>>: _BitStrLit,
{
    type Ret = B<NumRet<MulExp<LB, Val>>, _0>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U1, U2, U3, U4, U5, U6, U7, U8},
        test_res::*,
    };
    #[allow(clippy::used_underscore_items)]
    #[allow(non_upper_case_globals)]
    #[test]
    fn eval_mul() {
        const _0_MUL_0: () = _b0::<MulExp<U0, U0>>();
        const _0_MUL_1: () = _b0::<MulExp<U0, U1>>();
        const _1_MUL_0: () = _b0::<MulExp<U1, U0>>();
        const _2_MUL_1: () = _b2::<MulExp<U2, U1>>();
        const _1_MUL_2: () = _b2::<MulExp<U1, U2>>();
        const _3_MUL_1: () = _b3::<MulExp<U3, U1>>();
        const _1_MUL_3: () = _b3::<MulExp<U1, U3>>();
        const _2_MUL_2: () = _b4::<MulExp<U2, U2>>();
        const _4_MUL_1: () = _b4::<MulExp<U4, U1>>();
        const _5_MUL_1: () = _b5::<MulExp<U5, U1>>();
        const _6_MUL_1: () = _b6::<MulExp<U6, U1>>();
        const _7_MUL_1: () = _b7::<MulExp<U7, U1>>();
        const _8_MUL_1: () = _b8::<MulExp<U8, U1>>();
        const _3_MUL_3: () = _b9::<MulExp<U3, U3>>();

        const _2_MUL_2__MUL_2: () = _b8::<MulExp<MulExp<U2, U2>, U2>>();
        const _2_MUL__2_MUL_2: () = _b8::<MulExp<U2, MulExp<U2, U2>>>();
    }
}
