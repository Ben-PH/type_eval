
use crate::{
    op_types::Sub,
    val_types::{BitStrLit, BitString, Number, _0, _1},
    Eval, Expr,
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



// ---
// Carrying increment to a bit-string-literal
// ---
impl<B> Expr for Sub<BitString<B, _1>, _1, Eval>
where
    B: BitStrLit,
{
    type Output = BitString<B, _0>;
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
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _0>, Eval>
where
    LB: BitStrLit,
    RB: BitStrLit,
    Sub<LB, RB, Eval>: Expr,
    BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>: Number,
{
    type Output = BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>;
}
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _1>, Eval>
where
    LB: BitStrLit,
    RB: BitStrLit,
    Sub<LB, RB, Eval>: Expr,
    Sub<<Sub<LB, RB, Eval> as Expr>::Output, _1>: Expr,
    
    BitString<<<Sub<<Sub<LB, RB, Eval> as Expr>::Output, _1> as Expr>::Output as Expr>::Output, _1>: Number,
{
    type Output = BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>;
}
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _0>, Eval>
where
    LB: BitStrLit,
    RB: BitStrLit,
    Sub<LB, RB, Eval>: Expr,
    BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>: Number,
{
    type Output = BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>;
}
impl<LB, RB> Expr for Sub<BitString<LB, _0>, BitString<RB, _0>, Eval>
where
    LB: BitStrLit,
    RB: BitStrLit,
    Sub<LB, RB, Eval>: Expr,
    BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>: Number,
{
    type Output = BitString<<Sub<LB, RB, Eval> as Expr>::Output, _0>;
}
#[cfg(test)]
mod test {
    use super::*;
    const fn _b0<E: Expr<Output = _0>>() {}
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
        // const _2_SUB_2: () = _b0::<Sub<BitString<_1, _0>, BitString<_1, _0>>>();
        // const _3_SUB_2: () = _b1::<Sub<BitString<_1, _1>, BitString<_1, _0>>>();
        // const _3_SUB_3: () = _b0::<Sub<BitString<_1, _1>, BitString<_1, _1>>>();
    }
}
