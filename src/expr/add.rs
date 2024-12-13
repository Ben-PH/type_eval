use crate::{
    op_types::Add,
    val_types::{BitStrLit, BitString, _0, _1},
    Base, ExpRet, Expr,
};

impl<L, R> Expr for Add<L, R>
where
    Add<L::Ret, R::Ret, Base>: Expr,
    L: Expr,
    R: Expr,
{
    type Ret = <Add<L::Ret, R::Ret, Base> as Expr>::Ret;
}

// ----
//  Most basic of addition evaluations
// ----
impl Expr for Add<_0, _0, Base> {
    type Ret = _0;
}
impl Expr for Add<_0, _1, Base> {
    type Ret = _1;
}
impl Expr for Add<_1, _0, Base> {
    type Ret = _1;
}
impl Expr for Add<_1, _1, Base> {
    type Ret = BitString<_1, _0>;
}

// ---
// Non-carry bit-additions to bit-string literal
// ---
impl Expr for Add<BitString<_1, _0>, _1, Base> {
    type Ret = BitString<_1, _1>;
}
impl Expr for Add<_1, BitString<_1, _0>, Base> {
    type Ret = BitString<_1, _1>;
}

// ---
// Carrying increment to a bit-string-literal
// ---
impl<B> Expr for Add<BitString<B, _1>, _1, Base>
where
    // Recurse the carry
    Add<B, _1>: Expr,
    // Ensure the carry recursion is a valid progression
    ExpRet<Add<B, _1>>: BitStrLit,
{
    type Ret = BitString<ExpRet<Add<B, _1>>, _0>;
}

impl<B> Expr for Add<_1, BitString<B, _1>, Base>
where
    Add<B, _1>: Expr,
    ExpRet<Add<B, _1>>: BitStrLit,
{
    type Ret = BitString<ExpRet<Add<B, _1>>, _0>;
}

// ---
// Addition of two bit-string literals
// ---
/// (LB, 0) + (RB, 0) == ((LB + RB), 0)
impl<LB, RB> Expr for Add<BitString<LB, _0>, BitString<RB, _0>, Base>
where
    Add<LB, RB>: Expr,
    ExpRet<Add<LB, RB>>: BitStrLit,
{
    type Ret = BitString<ExpRet<Add<LB, RB>>, _0>;
}
/// (LB, 0) + (RB, 1) == ((LB + RB), 1)
impl<LB, RB> Expr for Add<BitString<LB, _0>, BitString<RB, _1>, Base>
where
    Add<LB, RB>: Expr,
    ExpRet<Add<LB, RB>>: BitStrLit,
{
    type Ret = BitString<ExpRet<Add<LB, RB>>, _1>;
}
impl<LB, RB> Expr for Add<BitString<LB, _1>, BitString<RB, _0>, Base>
where
    Add<LB, RB>: Expr,
    ExpRet<Add<LB, RB>>: BitStrLit,
{
    type Ret = BitString<ExpRet<Add<LB, RB>>, _1>;
}
/// (LB, 1) + (RB, 1) == ((LB + RB) + 1, 1)
impl<LB, RB> Expr for Add<BitString<LB, _1>, BitString<RB, _1>, Base>
where
    Add<LB, RB>: Expr,
    ExpRet<Add<LB, RB>>: Expr,
    Add<ExpRet<ExpRet<Add<LB, RB>>>, _1>: Expr,
    ExpRet<Add<ExpRet<ExpRet<Add<LB, RB>>>, _1>>: BitStrLit,
{
    type Ret = BitString<ExpRet<Add<ExpRet<ExpRet<Add<LB, RB>>>, _1>>, _0>;
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::test_res::*;
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
        const _7_ADD_1: () = _b8::<Add<BitString<BitString<_1, _1>, _1>, _1>>();
    }
}
