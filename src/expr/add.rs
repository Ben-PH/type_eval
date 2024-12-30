use crate::{
    num_vals::{U0, U1},
    op_types::AddExp,
    val_types::{NumberVal, B, _0, _1},
    NumExpr,
    _inners::_BitStrLit,
};
use core::ops::Add as StAdd;

pub type StAddRet<L, R> = <L as StAdd<R>>::Output;

impl<L, R> NumExpr for AddExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    L::Ret: StAdd<R::Ret>,
    StAddRet<L::Ret, R::Ret>: NumberVal,
{
    type Ret = StAddRet<L::Ret, R::Ret>;
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
    type Output = B<U1, _0>;
    fn add(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}

// ---
// Non-carry bit-additions to bit-string literal
// ---
impl<Bs> StAdd<U1> for B<Bs, _0>
where
    Bs: _BitStrLit,
{
    type Output = B<Bs, _1>;
    fn add(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl<Bs> StAdd<B<Bs, _0>> for U1
where
    Bs: _BitStrLit,
{
    type Output = B<Bs, _1>;
    fn add(self, _rhs: B<Bs, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}

// ---
// Carrying increment to a bit-string-literal
// ---
impl<Bs> StAdd<U1> for B<Bs, _1>
where
    // Recurse the carry
    Bs: StAdd<U1>,
    // Ensure the carry recursion is a valid progression
    StAddRet<Bs, U1>: _BitStrLit,
{
    type Output = B<StAddRet<Bs, U1>, _0>;
    fn add(self, _rhs: U1) -> Self::Output {
        unimplemented!("type eval only")
    }
}

impl<Bs> StAdd<B<Bs, _1>> for U1
where
    Bs: StAdd<U1>,
    StAddRet<Bs, U1>: _BitStrLit,
{
    type Output = B<StAddRet<Bs, U1>, _0>;
    fn add(self, _rhs: B<Bs, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}

// ---
// Addition of two bit-string literals
// ---
/// (LB, 0) + (RB, 0) == ((LB + RB), 0)
impl<LB, RB> StAdd<B<RB, _0>> for B<LB, _0>
where
    LB: StAdd<RB>,
    StAddRet<LB, RB>: _BitStrLit,
{
    type Output = B<StAddRet<LB, RB>, _0>;
    fn add(self, _rhs: B<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (LB, 0) + (RB, 1) == ((LB + RB), 1)
impl<LB, RB> StAdd<B<RB, _1>> for B<LB, _0>
where
    LB: StAdd<RB>,
    StAddRet<LB, RB>: _BitStrLit,
{
    type Output = B<StAddRet<LB, RB>, _1>;
    fn add(self, _rhs: B<RB, _1>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
impl<LB, RB> StAdd<B<RB, _0>> for B<LB, _1>
where
    LB: StAdd<RB>,
    StAddRet<LB, RB>: _BitStrLit,
{
    type Output = B<StAddRet<LB, RB>, _1>;
    fn add(self, _rhs: B<RB, _0>) -> Self::Output {
        unimplemented!("type eval only")
    }
}
/// (LB, 1) + (RB, 1) == ((LB + RB) + 1, 1)
impl<LB, RB> StAdd<B<RB, _1>> for B<LB, _1>
where
    LB: StAdd<RB>,
    StAddRet<LB, RB>: _BitStrLit + StAdd<U1>,
    StAddRet<StAddRet<LB, RB>, U1>: _BitStrLit,
    // AddExp<NumRet<NumRet<AddExp<LB, RB>>>, U1>: NumExpr,
    // NumRet<AddExp<NumRet<NumRet<AddExp<LB, RB>>>, U1>>: BitStrLit,
{
    type Output = B<StAddRet<StAddRet<LB, RB>, U1>, _0>;
    fn add(self, _rhs: B<RB, _1>) -> Self::Output {
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
    #[allow(non_upper_case_globals)]
    #[allow(clippy::used_underscore_items)]
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
