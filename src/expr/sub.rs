use crate::{
    op_types::Sub,
    val_types::{BitStrLit, BitString, _0, _1},
    Eval, Expr, ExprOut,
};

impl<L, R> Expr for Sub<L, R>
where
    L: Expr,
    R: Expr,
    Sub<L::Output, R::Output, Eval>: Expr,
{
    type Output = <Sub<L::Output, R::Output, Eval> as Expr>::Output;
}

// ----
//  Most basic of subtraction evaluations
// ----
impl Expr for Sub<_0, _0, Eval> {
    type Output = _0;
}
impl Expr for Sub<_1, _0, Eval> {
    type Output = _1;
}
impl Expr for Sub<_1, _1, Eval> {
    type Output = _0;
}
// Commented out as we are not yet handling negatives
// impl Expr for Sub<_0, _1, Eval> {
//     type Output = _1;
// }

// ---
// Hard-codeable trimmed decrement
// ---
impl Expr for Sub<BitString<_1, _0>, _1, Eval> {
    type Output = _1;
}

// Commented out as we are not yet handling negatives
// impl Expr for Sub<_1, BitString<_1, _0>, Eval> {
//     type Output = BitString<_1, _1>;
// }

/// Non-trimming decrement
impl<B> Expr for Sub<BitString<B, _1>, _1, Eval>
where
    B: Expr,
    ExprOut<B>: BitStrLit,
{
    type Output = BitString<B::Output, _0>;
}

/// (L)0 - (R)0 = (L - R)0
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _0>, Eval>
where
    // `L - R` is valid
    Sub<LB, RB>: Expr,
    // `(L - R)0` is valid as a bitstring
    BitString<ExprOut<Sub<LB, RB>>, _0>: Expr,
{
    type Output = ExprOut<BitString<ExprOut<Sub<LB, RB>>, _0>>;
}
/// (L)0 - (R)1 = ((L - R) - 1)1
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _1>, Eval>
where
    // `L - R` is valid
    Sub<LB, RB>: Expr,
    // `(L - R) - 1` is a valid expression
    Sub<ExprOut<Sub<LB, RB>>, _1>: Expr,
    // `(L - R) - 1` Is a valid bit-string literal so can be appended with a bit to make a valid
    // number
    ExprOut<Sub<ExprOut<Sub<LB, RB>>, _1>>: BitStrLit,
{
    type Output = BitString<ExprOut<Sub<ExprOut<Sub<LB, RB>>, _1>>, _1>;
}
/// (L)1 - (R)0 = (L - R)1
impl<LB, RB> Expr for Sub<BitString<LB, _1>, BitString<RB, _0>, Eval>
where
    // `L - R` is valid
    Sub<LB, RB>: Expr,
    // `(L - R)1` is valid as a bitstring
    BitString<ExprOut<Sub<LB, RB>>, _1>: Expr,
{
    type Output = ExprOut<BitString<ExprOut<Sub<LB, RB>>, _1>>;
}

/// (L)1 - (R)1 = (L - R)0
impl<LB, RB> Expr for Sub<BitString<LB, _1>, BitString<RB, _1>, Eval>
where
    // same operation as (L)0 - (R)0
    Sub<BitString<LB, _0>, BitString<RB, _0>>: Expr,
{
    type Output = ExprOut<Sub<BitString<LB, _0>, BitString<RB, _0>>>;
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
