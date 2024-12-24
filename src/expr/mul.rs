use crate::{
    num_vals::{U0, U1},
    op_types::{AddExp, MulExp},
    val_types::{BitLit, BitStrLit, NumberVal, _0, _1},
    Base, BitString, NumExpr, NumRet,
};

impl<L, R> NumExpr for MulExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    MulExp<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = NumRet<MulExp<L::Ret, R::Ret, Base>>;
}
// -------------
// Hard codod base cases.
// Past attempts to coallesce these impls resulted in overlapping-impl headaches
// -------------

impl NumExpr for MulExp<U0, U1, Base> {
    type Ret = U0;
}
impl NumExpr for MulExp<U1, U0, Base> {
    type Ret = U0;
}
impl NumExpr for MulExp<U0, U0, Base> {
    type Ret = U0;
}
impl NumExpr for MulExp<U1, U1, Base> {
    type Ret = U1;
}
impl<Bs, B> NumExpr for MulExp<U1, BitString<Bs, B>, Base>
where
    Bs: NumExpr,
    B: BitLit,
    BitString<Bs, B>: NumberVal,
{
    type Ret = BitString<Bs, B>;
}

// (LB, U1) * Val == ((LB * Val), U0) + Val
impl<LB, Val> NumExpr for MulExp<BitString<LB, _1>, Val, Base>
where
    LB: BitStrLit,
    MulExp<LB, Val>: NumExpr,
    AddExp<BitString<NumRet<MulExp<LB, Val>>, _0>, Val>: NumExpr,
{
    type Ret = NumRet<AddExp<BitString<NumRet<MulExp<LB, Val>>, _0>, Val>>;
}
// (LB, U0) * Val == ((LB * Val), U0)
impl<LB, Val> NumExpr for MulExp<BitString<LB, _0>, Val, Base>
where
    LB: BitStrLit,
    MulExp<LB, Val>: NumExpr,
    NumRet<MulExp<LB, Val>>: BitStrLit,
{
    type Ret = BitString<NumRet<MulExp<LB, Val>>, _0>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U1, U2, U3, U4, U5, U6, U7, U8},
        test_res::*,
    };
    #[test]
    fn eval_add() {
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
