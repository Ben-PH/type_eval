use crate::{
    op_types::{Add, Mul},
    val_types::{BitLit, BitStrLit, BitString, NumberVal, _0, _1},
    Base, NumExpr, NumRet,
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

impl NumExpr for Mul<_0, _1, Base> {
    type Ret = _0;
}
impl NumExpr for Mul<_1, _0, Base> {
    type Ret = _0;
}
impl NumExpr for Mul<_0, _0, Base> {
    type Ret = _0;
}
impl NumExpr for Mul<_1, _1, Base> {
    type Ret = _1;
}
impl<Bs, B> NumExpr for Mul<_1, BitString<Bs, B>, Base>
where
    Bs: NumExpr,
    B: BitLit,
    BitString<Bs, B>: NumberVal,
{
    type Ret = BitString<Bs, B>;
}

// (LB, _1) * Val == ((LB * Val), _0) + Val
impl<LB, Val> NumExpr for Mul<BitString<LB, _1>, Val, Base>
where
    LB: BitStrLit,
    Mul<LB, Val>: NumExpr,
    Add<BitString<NumRet<Mul<LB, Val>>, _0>, Val>: NumExpr,
{
    type Ret = NumRet<Add<BitString<NumRet<Mul<LB, Val>>, _0>, Val>>;
}
// (LB, _0) * Val == ((LB * Val), _0)
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
    use crate::test_res::*;
    #[test]
    fn eval_add() {
        const _0_MUL_0: () = _b0::<Mul<_0, _0>>();
        const _0_MUL_1: () = _b0::<Mul<_0, _1>>();
        const _1_MUL_0: () = _b0::<Mul<_1, _0>>();
        const _2_MUL_1: () = _b2::<Mul<BitString<_1, _0>, _1>>();
        const _1_MUL_2: () = _b2::<Mul<_1, BitString<_1, _0>>>();
        const _3_MUL_1: () = _b3::<Mul<BitString<_1, _1>, _1>>();
        const _1_MUL_3: () = _b3::<Mul<_1, BitString<_1, _1>>>();
        const _2_MUL_2: () = _b4::<Mul<BitString<_1, _0>, BitString<_1, _0>>>();
        const _4_MUL_1: () = _b4::<Mul<BitString<BitString<_1, _0>, _0>, _1>>();
        const _5_MUL_1: () = _b5::<Mul<BitString<BitString<_1, _0>, _1>, _1>>();
        const _6_MUL_1: () = _b6::<Mul<BitString<BitString<_1, _1>, _0>, _1>>();
        const _7_MUL_1: () = _b7::<Mul<BitString<BitString<_1, _1>, _1>, _1>>();
        const _8_MUL_1: () = _b8::<Mul<BitString<BitString<BitString<_1, _0>, _0>, _0>, _1>>();
        const _3_MUL_3: () = _b9::<Mul<BitString<_1, _1>, BitString<_1, _1>>>();
    }
}
