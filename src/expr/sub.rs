use crate::{
    num_vals::{U0, U1},
    op_types::SubExp,
    val_types::{NumberVal, B, _0, _1},
    NumExpr, NumRet,
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
impl<LBs, LB> StSub<U0> for B<LBs, LB> {
    type Output = B<LBs, LB>;
    fn sub(self, _rhs: U0) -> Self::Output {
        unimplemented!("type eval only")
    }
}
// Commented out as we are not yet handling negatives
// impl Expr for Sub<_0, U1, Base> {
//     type Ret = U1;
// }

impl<Bs> StSub<_1> for B<Bs, _1> {
    type Output = B<Bs, _0>;
    fn sub(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}

impl<Bs> StSub<_1> for B<Bs, _0>
where
    Bs: StSub<_1>,
    B<SubOut<Bs, _1>, _1>: NumExpr,
{
    type Output = NumRet<B<SubOut<Bs, _1>, _1>>;
    fn sub(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (L)0 - (R)0 = (L - R)0
impl<LB, RB> StSub<B<RB, _0>> for B<LB, _0>
where
    // `L - R` is valid
    LB: StSub<RB>,
    // `(L - R)0` is valid as a bitstring
    B<SubOut<LB, RB>, _0>: NumExpr,
{
    type Output = NumRet<B<SubOut<LB, RB>, _0>>;
    fn sub(self, _rhs: B<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (L)0 - (R)1 = ((L - R) - 1)1
impl<LB, RB> StSub<B<RB, _1>> for B<LB, _0>
where
    // `L - R` is valid
    LB: StSub<RB>,
    // `(L - R) - 1` is a valid expression
    SubOut<LB, RB>: StSub<_1>,
    // `(L - R) - 1` Is a valid bit-string literal so can be appended with a bit to make a valid
    // number
    B<SubOut<SubOut<LB, RB>, _1>, _1>: NumExpr,
{
    type Output = NumRet<B<SubOut<SubOut<LB, RB>, _1>, _1>>;
    fn sub(self, _rhs: B<RB, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (L)1 - (R)0 = (L - R)1
impl<LB, RB> StSub<B<RB, _0>> for B<LB, _1>
where
    // `L - R` is valid
    LB: StSub<RB>,
    B<SubOut<LB, RB>, _1>: NumExpr,
    // `(L - R)1` is valid as a bitstring
    // B<NumRet<SubExp<LB, RB>>, _1>: NumExpr,
{
    type Output = NumRet<B<SubOut<LB, RB>, _1>>;
    fn sub(self, _rhs: B<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}

/// (L)1 - (R)1 = (L - R)0
impl<LB, RB> StSub<B<RB, _1>> for B<LB, _1>
where
    // `L - R` is valid
    LB: StSub<RB>,
    // `(L - R)0` is valid as a bitstring
    B<SubOut<LB, RB>, _0>: NumExpr,
{
    type Output = NumRet<B<SubOut<LB, RB>, _0>>;
    fn sub(self, _rhs: B<RB, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3},
        test_res::*,
    };
    #[test]
    #[allow(clippy::used_underscore_items)]
    fn eval_sub() {
        const _0_SUB_0: () = _b0::<SubExp<U0, U0>>();
        const _1_SUB_0: () = _b1::<SubExp<U1, U0>>();
        const _1_SUB_1: () = _b0::<SubExp<U1, U1>>();
        const _2_SUB_1: () = _b1::<SubExp<U2, U1>>();
        const _2_SUB_0: () = _b2::<SubExp<U2, U0>>();
        // const _1_SUB_2: () = _b3::<SubExp<_1, B<_1, _0>>>();
        const _3_SUB_1: () = _b2::<SubExp<U3, _1>>();
        const _4_SUB_1: () = _b3::<SubExp<B<B<_1, _0>, _0>, _1>>();
        // const _1_SUB_3: () = _b4::<SubExp<_1, B<_1, _1>>>();
        const _2_SUB_2: () = _b0::<SubExp<U2, U2>>();
        const _3_SUB_2: () = _b1::<SubExp<U3, U2>>();
        const _3_SUB_3: () = _b0::<SubExp<U3, U3>>();
    }
}
