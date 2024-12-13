use crate::{
    op_types::Sub,
    val_types::{BitStrLit, BitString, Number, _0, _1},
    Eval, Expr, ExprOut,
};

impl<L: Expr, R: Expr> Expr for Sub<L, R>
where
    Sub<L::Output, R::Output, Eval>: Expr,
{
    type Output = <Sub<L::Output, R::Output, Eval> as Expr>::Output;
}

// ----
//  Most basic of addition evaluations
// ----
impl Expr for Sub<_0, _0, Eval> {
    type Output = _0;
}
// impl Expr for Sub<_0, _1, Eval> {
//     type Output = _1;
// }
impl Expr for Sub<_1, _0, Eval> {
    type Output = _1;
}
// <Sub<_1, _1>>();
impl Expr for Sub<_1, _1, Eval> {
    type Output = _0;
}

// ---
// Non-carry bit-additions to bit-string literal
// ---
impl Expr for Sub<BitString<_1, _0>, _1, Eval> {
    type Output = _1;
}
// impl Expr for Sub<_1, BitString<_1, _0>, Eval> {
//     type Output = BitString<_1, _1>;
// }

impl<B> Expr for Sub<BitString<B, _1>, _1, Eval>
where
    B: Expr,
    ExprOut<B>: BitStrLit,
{
    type Output = BitString<B::Output, _0>;
}

// impl<B> Expr for Sub<_1, BitString<B, _1>, Eval>
// where
//     B: BitStrLit,
//     Sub<B, _1>: Expr,
//     <Sub<B, _1> as Expr>::Output: BitStrLit,
// {
//     type Output = BitString<<Sub<B, _1> as Expr>::Output, _0>;
// }

// ---
// Subition of two bit-string literals
// ---
// <Sub<BitString<_1, _0>, BitString<_1, _0>>>();
// impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _0>, Eval>
// where
//     LB: BitStrLit,
//     RB: BitStrLit,
//     Sub<LB, RB, Eval>: Expr,
//     BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>: Number,
// {
//     type Output = BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>;
// }

// (L)0 - (R)0 = (L - R)0
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _0>, Eval>
where
    // Can L - R
    Sub<LB, RB>: Expr,
    // L - R can form the head of a BitString
    BitString<ExprOut<Sub<LB, RB>>, _0>: Expr,
{
    type Output = ExprOut<BitString<ExprOut<Sub<LB, RB>>, _0>>;
}
// (L)0 - (R)1 = ((L - R) - 1)1
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _1>, Eval>
where
    // can do L - R
    Sub<LB, RB>: Expr,
    // can do (L - R) - 1
    Sub<ExprOut<Sub<LB, RB>>, _1>: Expr,
    // (L - R) - 1 can form the head of a bitstring
    ExprOut<Sub<ExprOut<Sub<LB, RB>>, _1>>: BitStrLit,
{
    type Output = BitString<ExprOut<Sub<ExprOut<Sub<LB, RB>>, _1>>, _1>;
}
// (L)1 - (R)0 = (L - R)1
impl<LB, RB> Expr for Sub<BitString<LB, _1>, BitString<RB, _0>, Eval>
where
    // can do L - R
    Sub<LB, RB>: Expr,
    // L - R can be the head of a bitstring
    BitString<ExprOut<Sub<LB, RB>>, _1>: Expr,
{
    type Output = ExprOut<BitString<ExprOut<Sub<LB, RB>>, _1>>;
}
// (L)1 - (R)1 = (L - R)0
impl<LB, RB> Expr for Sub<BitString<LB, _1>, BitString<RB, _1>, Eval>
where
    Sub<BitString<LB, _0>, BitString<RB, _0>>: Expr,
{
    type Output = ExprOut<Sub<BitString<LB, _0>, BitString<RB, _0>>>;
}

#[cfg(test)]
mod test {
    use super::*;
    const fn _b0<E: Expr<Output = _0>>() {}
    // const fn _b00<E: Expr<Output = BitString<_0, _0>>>() {}
    const fn _b1<E: Expr<Output = _1>>() {}
    const fn _b2<E: Expr<Output = BitString<_1, _0>>>() {}
    const fn _b3<E: Expr<Output = BitString<_1, _1>>>() {}
    const fn _b4<E: Expr<Output = BitString<BitString<_1, _0>, _0>>>() {}
    const fn _b5<E: Expr<Output = BitString<BitString<_1, _0>, _1>>>() {}
    const fn _b6<E: Expr<Output = BitString<BitString<_1, _1>, _0>>>() {}
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
