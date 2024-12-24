use crate::{
    num_vals::{U0, U1},
    op_types::AddExp,
    val_types::{BitStrLit, NumberVal, _0, _1},
    BitString, NumExpr,
};
use core::ops::Add as StAdd;

pub type AddOut<L, R> = <L as StAdd<R>>::Output;

impl<L, R> NumExpr for AddExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    L::Ret: StAdd<R::Ret>,
    AddOut<L::Ret, R::Ret>: NumberVal,
{
    type Ret = AddOut<L::Ret, R::Ret>;
}

// ----
//  Most basic of addition evaluations
// ----
impl StAdd<U0> for U0 {
    type Output = U0;

    fn add(self, _rhs: U0) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl StAdd<U1> for U0 {
    type Output = U1;
    fn add(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl StAdd<U0> for U1 {
    type Output = U1;
    fn add(self, _rhs: U0) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl StAdd<U1> for U1 {
    type Output = BitString<U1, _0>;
    fn add(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}

// ---
// Non-carry bit-additions to bit-string literal
// ---
impl<B> StAdd<U1> for BitString<B, _0>
where
    B: BitStrLit,
{
    type Output = BitString<B, _1>;
    fn add(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl<B> StAdd<BitString<B, _0>> for U1
where
    B: BitStrLit,
{
    type Output = BitString<B, _1>;
    fn add(self, _rhs: BitString<B, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}

// ---
// Carrying increment to a bit-string-literal
// ---
impl<B> StAdd<U1> for BitString<B, _1>
where
    // Recurse the carry
    B: StAdd<U1>,
    // Ensure the carry recursion is a valid progression
    AddOut<B, U1>: BitStrLit,
{
    type Output = BitString<AddOut<B, U1>, _0>;
    fn add(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}

impl<B> StAdd<BitString<B, _1>> for U1
where
    B: StAdd<U1>,
    AddOut<B, U1>: BitStrLit,
{
    type Output = BitString<AddOut<B, U1>, _0>;
    fn add(self, _rhs: BitString<B, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}

// ---
// Addition of two bit-string literals
// ---
/// (LB, 0) + (RB, 0) == ((LB + RB), 0)
impl<LB, RB> StAdd<BitString<RB, _0>> for BitString<LB, _0>
where
    LB: StAdd<RB>,
    AddOut<LB, RB>: BitStrLit,
{
    type Output = BitString<AddOut<LB, RB>, _0>;
    fn add(self, _rhs: BitString<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (LB, 0) + (RB, 1) == ((LB + RB), 1)
impl<LB, RB> StAdd<BitString<RB, _1>> for BitString<LB, _0>
where
    LB: StAdd<RB>,
    AddOut<LB, RB>: BitStrLit,
{
    type Output = BitString<AddOut<LB, RB>, _1>;
    fn add(self, _rhs: BitString<RB, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl<LB, RB> StAdd<BitString<RB, _0>> for BitString<LB, _1>
where
    LB: StAdd<RB>,
    AddOut<LB, RB>: BitStrLit,
{
    type Output = BitString<AddOut<LB, RB>, _1>;
    fn add(self, _rhs: BitString<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (LB, 1) + (RB, 1) == ((LB + RB) + 1, 1)
impl<LB, RB> StAdd<BitString<RB, _1>> for BitString<LB, _1>
where
    LB: StAdd<RB>,
    AddOut<LB, RB>: BitStrLit + StAdd<U1>,
    AddOut<AddOut<LB, RB>, U1>: BitStrLit,
    // AddExp<NumRet<NumRet<AddExp<LB, RB>>>, U1>: NumExpr,
    // NumRet<AddExp<NumRet<NumRet<AddExp<LB, RB>>>, U1>>: BitStrLit,
{
    type Output = BitString<AddOut<AddOut<LB, RB>, U1>, _0>;
    fn add(self, _rhs: BitString<RB, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_add() {
        const _0_ADD_0: () = _b0::<AddExp<U0, U0>>();
        const _1_ADD_0: () = _b1::<AddExp<U1, U0>>();
        const _1_ADD_1: () = _b2::<AddExp<U1, U1>>();
        const _2_ADD_1: () = _b3::<AddExp<U2, U1>>();
        const _1_ADD_2: () = _b3::<AddExp<U1, U2>>();
        const _3_ADD_1: () = _b4::<AddExp<U3, U1>>();
        const _4_ADD_1: () = _b5::<AddExp<U4, U1>>();
        const _1_ADD_3: () = _b4::<AddExp<U1, U3>>();
        const _1_ADD_4: () = _b5::<AddExp<U1, U4>>();
        const _2_ADD_2: () = _b4::<AddExp<U2, U2>>();
        const _3_ADD_3: () = _b6::<AddExp<U3, U3>>();
        const _6_ADD_1: () = _b7::<AddExp<U6, U1>>();
        const _7_ADD_1: () = _b8::<AddExp<U7, U1>>();

        const _1_ADD_1__ADD_1: () = _b3::<AddExp<U1, AddExp<U1, U1>>>();
        const _1_ADD__1_ADD_1: () = _b3::<AddExp<AddExp<U1, U1>, U1>>();

        const _3_ADD_3__ADD_3: () = _b9::<AddExp<U3, AddExp<U3, U3>>>();
        const _3_ADD__3_ADD_3: () = _b9::<AddExp<AddExp<U3, U3>, U3>>();
    }
}
