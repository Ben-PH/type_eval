use crate::{
    op_types::{Add, Mul},
    val_types::{BitLit, BitStrLit, BitString, NumberVal, _0, _1},
    Base, NumExpr, NumRet, U0, U1,
};

impl<L, R> NumExpr for Mul<L, R>
where
    L: NumExpr,
    R: NumExpr,
    Mul<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = NumRet<Mul<L::Ret, R::Ret, Base>>;
}
// -------------
// Hard codod base cases.
// Past attempts to coallesce these impls resulted in overlapping-impl headaches
// -------------

impl NumExpr for Mul<U0, U1, Base> {
    type Ret = U0;
}
impl NumExpr for Mul<U1, U0, Base> {
    type Ret = U0;
}
impl NumExpr for Mul<U0, U0, Base> {
    type Ret = U0;
}
impl NumExpr for Mul<U1, U1, Base> {
    type Ret = U1;
}
impl<Bs, B> NumExpr for Mul<U1, BitString<Bs, B>, Base>
where
    Bs: NumExpr,
    B: BitLit,
    BitString<Bs, B>: NumberVal,
{
    type Ret = BitString<Bs, B>;
}

// (LB, U1) * Val == ((LB * Val), U0) + Val
impl<LB, Val> NumExpr for Mul<BitString<LB, _1>, Val, Base>
where
    LB: BitStrLit,
    Mul<LB, Val>: NumExpr,
    Add<BitString<NumRet<Mul<LB, Val>>, _0>, Val>: NumExpr,
{
    type Ret = NumRet<Add<BitString<NumRet<Mul<LB, Val>>, _0>, Val>>;
}
// (LB, U0) * Val == ((LB * Val), U0)
impl<LB, Val> NumExpr for Mul<BitString<LB, _0>, Val, Base>
where
    LB: BitStrLit,
    Mul<LB, Val>: NumExpr,
    NumRet<Mul<LB, Val>>: BitStrLit,
{
    type Ret = BitString<NumRet<Mul<LB, Val>>, _0>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{test_res::*, U1, U2, U3, U4, U5, U6, U7, U8};
    #[test]
    fn eval_add() {
        const _0_MUL_0: () = _b0::<Mul<U0, U0>>();
        const _0_MUL_1: () = _b0::<Mul<U0, U1>>();
        const _1_MUL_0: () = _b0::<Mul<U1, U0>>();
        const _2_MUL_1: () = _b2::<Mul<U2, U1>>();
        const _1_MUL_2: () = _b2::<Mul<U1, U2>>();
        const _3_MUL_1: () = _b3::<Mul<U3, U1>>();
        const _1_MUL_3: () = _b3::<Mul<U1, U3>>();
        const _2_MUL_2: () = _b4::<Mul<U2, U2>>();
        const _4_MUL_1: () = _b4::<Mul<U4, U1>>();
        const _5_MUL_1: () = _b5::<Mul<U5, U1>>();
        const _6_MUL_1: () = _b6::<Mul<U6, U1>>();
        const _7_MUL_1: () = _b7::<Mul<U7, U1>>();
        const _8_MUL_1: () = _b8::<Mul<U8, U1>>();
        const _3_MUL_3: () = _b9::<Mul<U3, U3>>();

        const _2_MUL_2__MUL_2: () = _b8::<Mul<Mul<U2, U2>, U2>>();
        const _2_MUL__2_MUL_2: () = _b8::<Mul<U2, Mul<U2, U2>>>();
    }
}
