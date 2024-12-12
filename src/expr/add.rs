use crate::{
    op_types::Add,
    val_types::{BitStrLit, BitString, _0, _1},
    Eval, Expr,
};

impl<L: Expr, R: Expr> Expr for Add<L, R>
where
    Add<L::Output, R::Output, Eval>: Expr,
{
    type Output = <Add<L::Output, R::Output, Eval> as Expr>::Output;
}

// ----
//  Most basic of addition evaluations
// ----
impl Expr for Add<_0, _0, Eval> {
    type Output = _0;
}
impl Expr for Add<_0, _1, Eval> {
    type Output = _1;
}
impl Expr for Add<_1, _0, Eval> {
    type Output = _1;
}
impl Expr for Add<_1, _1, Eval> {
    type Output = BitString<_1, _0>;
}


// ---
// Non-carry bit-additions to bit-string literal
// ---
impl Expr for Add<BitString<_1, _0>, _1, Eval> {
    type Output = BitString<_1, _1>;
}
impl Expr for Add<_1, BitString<_1, _0>, Eval> {
    type Output = BitString<_1, _1>;
}



// ---
// Carrying increment to a bit-string-literal
// ---
impl<B:BitStrLit> Expr for Add<BitString<B, _1>, _1, Eval>
where
    // B: BitStrLit,
    Add<B, _1>: Expr,
    <Add<B, _1> as Expr>::Output: BitStrLit,
{
    type Output = BitString<<Add<B, _1> as Expr>::Output, _0>;
}

impl<B:BitStrLit> Expr for Add<_1, BitString<B, _1>, Eval>
where
    // B: BitStrLit,
    Add<B, _1>: Expr,
    <Add<B, _1> as Expr>::Output: BitStrLit,
{
    type Output = BitString<<Add<B, _1> as Expr>::Output, _0>;
}

// ---
// Addition of two bit-string literals
// ---
impl<LB:BitStrLit, RB:BitStrLit> Expr for Add<BitString<LB, _0>, BitString<RB, _0>, Eval>
where
    // LB: BitStrLit,
    // RB: BitStrLit,
    Add<LB, RB>: Expr,
    <Add<LB, RB> as Expr>::Output: BitStrLit,
{
    type Output = BitString<<Add<LB, RB> as Expr>::Output, _0>;
}
impl<LB : BitStrLit, RB : BitStrLit> Expr for Add<BitString<LB, _0>, BitString<RB, _1>, Eval>
where
    // LB: BitStrLit,
    // RB: BitStrLit,
    Add<LB, RB>: Expr,
    <Add<LB, RB> as Expr>::Output: BitStrLit,
{
    type Output = BitString<<Add<LB, RB> as Expr>::Output, _1>;
}
impl<LB : BitStrLit, RB : BitStrLit> Expr for Add<BitString<LB, _1>, BitString<RB, _0>, Eval>
where
    // LB: BitStrLit,
    // RB: BitStrLit,
    Add<LB, RB>: Expr,
    <Add<LB, RB> as Expr>::Output: BitStrLit,
{
    type Output = BitString<<Add<LB, RB> as Expr>::Output, _1>;
}
impl<LB : BitStrLit, RB :BitStrLit> Expr for Add<BitString<LB, _1>, BitString<RB, _1>, Eval>
where
    // LB: BitStrLit,
    // RB: BitStrLit,
    Add<LB, RB>: Expr,
    <Add<LB, RB> as Expr>::Output: Expr,
    Add<<<Add<LB, RB> as Expr>::Output as Expr>::Output, _1>: Expr,
    <Add<<<Add<LB, RB> as Expr>::Output as Expr>::Output, _1> as Expr>::Output: BitStrLit,
{
    type Output =
        BitString<<Add<<<Add<LB, RB> as Expr>::Output as Expr>::Output, _1> as Expr>::Output, _0>;
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
        const _0_ADD_0: () = _b0::<Add<_0, _0>>();
        const _1_ADD_0: () = _b1::<Add<_1, _0>>();
        const _1_ADD_1: () = _b2::<Add<_1, _1>>();
        const _2_ADD_1: () = _b3::<Add<BitString<_1, _0>, _1>>();
        const _1_ADD_2: () = _b3::<Add<_1, BitString<_1, _0>>>();
        const _3_ADD_1: () = _b4::<Add<BitString<_1, _1>, _1>>();
        const _1_ADD_3: () = _b4::<Add<_1, BitString<_1, _1>>>();
        const _2_ADD_2: () = _b4::<Add<BitString<_1, _0>, BitString<_1, _0>>>();
        const _3_ADD_3: () = _b6::<Add<BitString<_1, _1>, BitString<_1, _1>>>();
    }
}
