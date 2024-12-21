use crate::{
    op_types::Add,
    val_types::{BitStrLit, BitString, _0, _1},
    Base, NumExpr, NumRet, U0, U1,
};

impl<L, R> NumExpr for Add<L, R>
where
    L: NumExpr,
    R: NumExpr,
    Add<L::Ret, R::Ret, Base>: NumExpr,
{
    type Ret = NumRet<Add<L::Ret, R::Ret, Base>>;
}

// ----
//  Most basic of addition evaluations
// ----
impl NumExpr for Add<U0, U0, Base> {
    type Ret = U0;
}
impl NumExpr for Add<U0, U1, Base> {
    type Ret = U1;
}
impl NumExpr for Add<U1, U0, Base> {
    type Ret = U1;
}
impl NumExpr for Add<U1, U1, Base> {
    type Ret = BitString<U1, _0>;
}

// ---
// Non-carry bit-additions to bit-string literal
// ---
impl<B> NumExpr for Add<BitString<B, _0>, U1, Base>
where
    B: BitStrLit,
{
    type Ret = BitString<B, _1>;
}
impl<B> NumExpr for Add<U1, BitString<B, _0>, Base>
where
    B: BitStrLit,
{
    type Ret = BitString<U1, _1>;
}

// ---
// Carrying increment to a bit-string-literal
// ---
impl<B> NumExpr for Add<BitString<B, _1>, U1, Base>
where
    // Recurse the carry
    Add<B, U1>: NumExpr,
    // Ensure the carry recursion is a valid progression
    NumRet<Add<B, U1>>: BitStrLit,
{
    type Ret = BitString<NumRet<Add<B, U1>>, _0>;
}

impl<B> NumExpr for Add<U1, BitString<B, _1>, Base>
where
    Add<B, U1>: NumExpr,
    NumRet<Add<B, U1>>: BitStrLit,
{
    type Ret = BitString<NumRet<Add<B, U1>>, _0>;
}

// ---
// Addition of two bit-string literals
// ---
/// (LB, 0) + (RB, 0) == ((LB + RB), 0)
impl<LB, RB> NumExpr for Add<BitString<LB, _0>, BitString<RB, _0>, Base>
where
    Add<LB, RB>: NumExpr,
    NumRet<Add<LB, RB>>: BitStrLit,
{
    type Ret = BitString<NumRet<Add<LB, RB>>, _0>;
}
/// (LB, 0) + (RB, 1) == ((LB + RB), 1)
impl<LB, RB> NumExpr for Add<BitString<LB, _0>, BitString<RB, _1>, Base>
where
    Add<LB, RB>: NumExpr,
    NumRet<Add<LB, RB>>: BitStrLit,
{
    type Ret = BitString<NumRet<Add<LB, RB>>, _1>;
}
impl<LB, RB> NumExpr for Add<BitString<LB, _1>, BitString<RB, _0>, Base>
where
    Add<LB, RB>: NumExpr,
    NumRet<Add<LB, RB>>: BitStrLit,
{
    type Ret = BitString<NumRet<Add<LB, RB>>, _1>;
}
/// (LB, 1) + (RB, 1) == ((LB + RB) + 1, 1)
impl<LB, RB> NumExpr for Add<BitString<LB, _1>, BitString<RB, _1>, Base>
where
    Add<LB, RB>: NumExpr,
    NumRet<Add<LB, RB>>: NumExpr,
    Add<NumRet<NumRet<Add<LB, RB>>>, U1>: NumExpr,
    NumRet<Add<NumRet<NumRet<Add<LB, RB>>>, U1>>: BitStrLit,
{
    type Ret = BitString<NumRet<Add<NumRet<NumRet<Add<LB, RB>>>, U1>>, _0>;
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{test_res::*, U0, U1, U2, U3, U6, U7};
    #[test]
    fn eval_add() {
        const _0_ADD_0: () = _b0::<Add<U0, U0>>();
        const _1_ADD_0: () = _b1::<Add<U1, U0>>();
        const _1_ADD_1: () = _b2::<Add<U1, U1>>();
        const _2_ADD_1: () = _b3::<Add<U2, U1>>();
        const _1_ADD_2: () = _b3::<Add<U1, U2>>();
        const _3_ADD_1: () = _b4::<Add<U3, U1>>();
        const _1_ADD_3: () = _b4::<Add<U1, U3>>();
        const _2_ADD_2: () = _b4::<Add<U2, U2>>();
        const _3_ADD_3: () = _b6::<Add<U3, U3>>();
        const _6_ADD_1: () = _b7::<Add<U6, U1>>();
        const _7_ADD_1: () = _b8::<Add<U7, U1>>();

        const _1_ADD_1__ADD_1: () = _b3::<Add<U1, Add<U1, U1>>>();
        const _1_ADD__1_ADD_1: () = _b3::<Add<Add<U1, U1>, U1>>();

        const _3_ADD_3__ADD_3: () = _b9::<Add<U3, Add<U3, U3>>>();
        const _3_ADD__3_ADD_3: () = _b9::<Add<Add<U3, U3>, U3>>();
    }
}
