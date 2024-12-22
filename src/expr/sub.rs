use crate::{
    op_types::SubExp,
    val_types::{BitStrLit, BitString, _0, _1},
    Base, NumExpr, NumRet, U1,
};

impl<L, R> NumExpr for SubExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    SubExp<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = <SubExp<L::Ret, R::Ret, Base> as NumExpr>::Ret;
}

// ----
//  Most basic of subtraction evaluations
// ----
impl NumExpr for SubExp<_0, _0, Base> {
    type Ret = _0;
}
impl NumExpr for SubExp<U1, _0, Base> {
    type Ret = U1;
}
impl NumExpr for SubExp<U1, U1, Base> {
    type Ret = _0;
}
// Commented out as we are not yet handling negatives
// impl Expr for Sub<_0, U1, Base> {
//     type Ret = U1;
// }

// ---
// Hard-codeable trimmed decrement
// ---
impl NumExpr for SubExp<BitString<_1, _0>, _1, Base> {
    type Ret = _1;
}

// Commented out as we are not yet handling negatives
// impl Expr for Sub<_1, BitString<_1, _0>, Base> {
//     type Ret = BitString<_1, _1>;
// }

/// Non-trimming decrement
impl<B> NumExpr for SubExp<BitString<B, _1>, _1, Base>
where
    B: NumExpr,
    NumRet<B>: BitStrLit,
{
    type Ret = BitString<B::Ret, _0>;
}

/// (L)0 - (R)0 = (L - R)0
impl<LB, RB> NumExpr for SubExp<BitString<LB, _0>, BitString<RB, _0>, Base>
where
    // `L - R` is valid
    SubExp<LB, RB>: NumExpr,
    // `(L - R)0` is valid as a bitstring
    BitString<NumRet<SubExp<LB, RB>>, _0>: NumExpr,
{
    type Ret = NumRet<BitString<NumRet<SubExp<LB, RB>>, _0>>;
}
/// (L)0 - (R)1 = ((L - R) - 1)1
impl<LB, RB> NumExpr for SubExp<BitString<LB, _0>, BitString<RB, _1>, Base>
where
    // `L - R` is valid
    SubExp<LB, RB>: NumExpr,
    // `(L - R) - 1` is a valid expression
    SubExp<NumRet<SubExp<LB, RB>>, _1>: NumExpr,
    // `(L - R) - 1` Is a valid bit-string literal so can be appended with a bit to make a valid
    // number
    NumRet<SubExp<NumRet<SubExp<LB, RB>>, _1>>: BitStrLit,
{
    type Ret = BitString<NumRet<SubExp<NumRet<SubExp<LB, RB>>, _1>>, _1>;
}
/// (L)1 - (R)0 = (L - R)1
impl<LB, RB> NumExpr for SubExp<BitString<LB, _1>, BitString<RB, _0>, Base>
where
    // `L - R` is valid
    SubExp<LB, RB>: NumExpr,
    // `(L - R)1` is valid as a bitstring
    BitString<NumRet<SubExp<LB, RB>>, _1>: NumExpr,
{
    type Ret = NumRet<BitString<NumRet<SubExp<LB, RB>>, _1>>;
}

/// (L)1 - (R)1 = (L - R)0
impl<LB, RB> NumExpr for SubExp<BitString<LB, _1>, BitString<RB, _1>, Base>
where
    // same operation as (L)0 - (R)0
    SubExp<BitString<LB, _0>, BitString<RB, _0>>: NumExpr,
{
    type Ret = NumRet<SubExp<BitString<LB, _0>, BitString<RB, _0>>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{test_res::*, U0, U1, U2, U3};
    #[test]
    fn eval_add() {
        const _0_SUB_0: () = _b0::<SubExp<U0, U0>>();
        const _1_SUB_0: () = _b1::<SubExp<U1, U0>>();
        const _1_SUB_1: () = _b0::<SubExp<U1, U1>>();
        const _2_SUB_1: () = _b1::<SubExp<U2, U1>>();
        // const _1_SUB_2: () = _b3::<Sub<_1, BitString<_1, _0>>>();
        const _3_SUB_1: () = _b2::<SubExp<U3, _1>>();
        // const _4_SUB_1: () = _b3::<Sub<U4, _1>>();
        // const _1_SUB_3: () = _b4::<Sub<_1, BitString<_1, _1>>>();
        const _2_SUB_2: () = _b0::<SubExp<U2, U2>>();
        const _3_SUB_2: () = _b1::<SubExp<U3, U2>>();
        const _3_SUB_3: () = _b0::<SubExp<U3, U3>>();
    }
}
