use crate::{
    op_types::Sub,
    val_types::{BitStrLit, BitString, _0, _1},
    Base, ExpRet, Expr,
};

impl<L, R> Expr for Sub<L, R>
where
    L: Expr,
    R: Expr,
    Sub<L::Ret, R::Ret, Base>: Expr,
{
    type Ret = <Sub<L::Ret, R::Ret, Base> as Expr>::Ret;
}

// ----
//  Most basic of subtraction evaluations
// ----
impl Expr for Sub<_0, _0, Base> {
    type Ret = _0;
}
impl Expr for Sub<_1, _0, Base> {
    type Ret = _1;
}
impl Expr for Sub<_1, _1, Base> {
    type Ret = _0;
}
// Commented out as we are not yet handling negatives
// impl Expr for Sub<_0, _1, Base> {
//     type Ret = _1;
// }

// ---
// Hard-codeable trimmed decrement
// ---
impl Expr for Sub<BitString<_1, _0>, _1, Base> {
    type Ret = _1;
}

// Commented out as we are not yet handling negatives
// impl Expr for Sub<_1, BitString<_1, _0>, Base> {
//     type Ret = BitString<_1, _1>;
// }

/// Non-trimming decrement
impl<B> Expr for Sub<BitString<B, _1>, _1, Base>
where
    B: Expr,
    ExpRet<B>: BitStrLit,
{
    type Ret = BitString<B::Ret, _0>;
}

/// (L)0 - (R)0 = (L - R)0
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _0>, Base>
where
    // `L - R` is valid
    Sub<LB, RB>: Expr,
    // `(L - R)0` is valid as a bitstring
    BitString<ExpRet<Sub<LB, RB>>, _0>: Expr,
{
    type Ret = ExpRet<BitString<ExpRet<Sub<LB, RB>>, _0>>;
}
/// (L)0 - (R)1 = ((L - R) - 1)1
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _1>, Base>
where
    // `L - R` is valid
    Sub<LB, RB>: Expr,
    // `(L - R) - 1` is a valid expression
    Sub<ExpRet<Sub<LB, RB>>, _1>: Expr,
    // `(L - R) - 1` Is a valid bit-string literal so can be appended with a bit to make a valid
    // number
    ExpRet<Sub<ExpRet<Sub<LB, RB>>, _1>>: BitStrLit,
{
    type Ret = BitString<ExpRet<Sub<ExpRet<Sub<LB, RB>>, _1>>, _1>;
}
/// (L)1 - (R)0 = (L - R)1
impl<LB, RB> Expr for Sub<BitString<LB, _1>, BitString<RB, _0>, Base>
where
    // `L - R` is valid
    Sub<LB, RB>: Expr,
    // `(L - R)1` is valid as a bitstring
    BitString<ExpRet<Sub<LB, RB>>, _1>: Expr,
{
    type Ret = ExpRet<BitString<ExpRet<Sub<LB, RB>>, _1>>;
}

/// (L)1 - (R)1 = (L - R)0
impl<LB, RB> Expr for Sub<BitString<LB, _1>, BitString<RB, _1>, Base>
where
    // same operation as (L)0 - (R)0
    Sub<BitString<LB, _0>, BitString<RB, _0>>: Expr,
{
    type Ret = ExpRet<Sub<BitString<LB, _0>, BitString<RB, _0>>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_res::*;
    #[test]
    fn eval_add() {
        const _0_SUB_0: () = _b0::<Sub<_0, _0>>();
        const _1_SUB_0: () = _b1::<Sub<_1, _0>>();
        const _1_SUB_1: () = _b0::<Sub<_1, _1>>();
        const _2_SUB_1: () = _b1::<Sub<BitString<_1, _0>, _1>>();
        // const _1_SUB_2: () = _b3::<Sub<_1, BitString<_1, _0>>>();
        const _3_SUB_1: () = _b2::<Sub<BitString<_1, _1>, _1>>();
        // const _1_SUB_3: () = _b4::<Sub<_1, BitString<_1, _1>>>();
        const _2_SUB_2: () = _b0::<Sub<BitString<_1, _0>, BitString<_1, _0>>>();
        const _3_SUB_2: () = _b1::<Sub<BitString<_1, _1>, BitString<_1, _0>>>();
        const _3_SUB_3: () = _b0::<Sub<BitString<_1, _1>, BitString<_1, _1>>>();
    }
}
