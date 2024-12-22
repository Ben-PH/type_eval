use crate::{
    op_types::SubExp,
    val_types::{BitString, NumberVal, _0, _1},
    NumExpr, NumRet, U0, U1,
};
use core::ops::Sub as StSub;

impl<L, R> NumExpr for SubExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    L::Ret: StSub<R::Ret>,
    SubOut<L::Ret, R::Ret>: NumExpr,
    NumRet<SubOut<L::Ret, R::Ret>>: NumberVal,
{
    type Ret = NumRet<SubOut<L::Ret, R::Ret>>;
}

type SubOut<L, R> = <L as StSub<R>>::Output;

// ----
//  Most basic of subtraction evaluations
// ----
impl StSub<U0> for U0 {
    type Output = U0;

    fn sub(self, _rhs: U0) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl StSub<U0> for U1 {
    type Output = U1;
    fn sub(self, _rhs: U0) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl StSub<U1> for U1 {
    type Output = U0;
    fn sub(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}
// Commented out as we are not yet handling negatives
// impl Expr for Sub<_0, U1, Base> {
//     type Ret = U1;
// }

// ---
// Hard-codeable trimmed decrement
// ---
// impl StSub<_1> for BitString<_1, _0> {
//     type Output = U1;
//     fn sub(self, _rhs: U1) -> Self::Output {
//         unimplemented!("type eval only")
//     }
// }

// Commented out as we are not yet handling negatives
// impl Expr for Sub<_1, BitString<_1, _0>, Base> {
//     type Ret = BitString<_1, _1>;
// }

/// Non-trimming decrement
impl<B> StSub<_1> for BitString<B, _1> {
    type Output = BitString<B, _0>;
    fn sub(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}

impl<B> StSub<_1> for BitString<B, _0>
where
    B: StSub<_1>,
    BitString<SubOut<B, _1>, _1>: NumExpr,
{
    type Output = NumRet<BitString<SubOut<B, _1>, _1>>;
    fn sub(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (L)0 - (R)0 = (L - R)0
impl<LB, RB> StSub<BitString<RB, _0>> for BitString<LB, _0>
where
    // `L - R` is valid
    LB: StSub<RB>,
    // `(L - R)0` is valid as a bitstring
    BitString<SubOut<LB, RB>, _0>: NumExpr,
{
    type Output = NumRet<BitString<SubOut<LB, RB>, _0>>;
    fn sub(self, _rhs: BitString<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (L)0 - (R)1 = ((L - R) - 1)1
impl<LB, RB> StSub<BitString<RB, _1>> for BitString<LB, _0>
where
    // `L - R` is valid
    LB: StSub<RB>,
    // `(L - R) - 1` is a valid expression
    SubOut<LB, RB>: StSub<_1>,
    // `(L - R) - 1` Is a valid bit-string literal so can be appended with a bit to make a valid
    // number
    BitString<SubOut<SubOut<LB, RB>, _1>, _1>: NumExpr,
{
    type Output = NumRet<BitString<SubOut<SubOut<LB, RB>, _1>, _1>>;
    fn sub(self, _rhs: BitString<RB, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (L)1 - (R)0 = (L - R)1
impl<LB, RB> StSub<BitString<RB, _0>> for BitString<LB, _1>
where
    // `L - R` is valid
    LB: StSub<RB>,
    BitString<SubOut<LB, RB>, _1>: NumExpr,
    // `(L - R)1` is valid as a bitstring
    // BitString<NumRet<SubExp<LB, RB>>, _1>: NumExpr,
{
    type Output = NumRet<BitString<SubOut<LB, RB>, _1>>;
    fn sub(self, _rhs: BitString<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}

/// (L)1 - (R)1 = (L - R)0
impl<LB, RB> StSub<BitString<RB, _1>> for BitString<LB, _1>
where
    // `L - R` is valid
    LB: StSub<RB>,
    // `(L - R)0` is valid as a bitstring
    BitString<SubOut<LB, RB>, _0>: NumExpr,
{
    type Output = NumRet<BitString<SubOut<LB, RB>, _0>>;
    fn sub(self, _rhs: BitString<RB, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
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
        // const _1_SUB_2: () = _b3::<SubExp<_1, BitString<_1, _0>>>();
        const _3_SUB_1: () = _b2::<SubExp<U3, _1>>();
        const _4_SUB_1: () = _b3::<SubExp<BitString<BitString<_1, _0>, _0>, _1>>();
        // const _1_SUB_3: () = _b4::<SubExp<_1, BitString<_1, _1>>>();
        const _2_SUB_2: () = _b0::<SubExp<U2, U2>>();
        const _3_SUB_2: () = _b1::<SubExp<U3, U2>>();
        const _3_SUB_3: () = _b0::<SubExp<U3, U3>>();
    }
}
