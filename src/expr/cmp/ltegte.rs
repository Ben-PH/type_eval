use crate::{
    ctrl_types::{GT, GTE, LTE},
    num_vals::U1,
    op_types::AddExp,
    prelude::LT,
    BoolExpr, BoolRet, NumExpr, NumRet,
};

impl<L, R> BoolExpr for GTE<L, R>
where
    AddExp<L, U1>: NumExpr,
    GT<NumRet<AddExp<L, U1>>, R>: BoolExpr,
{
    type Ret = BoolRet<GT<NumRet<AddExp<L, U1>>, R>>;
}
impl<L, R> BoolExpr for LTE<L, R>
where
    AddExp<R, U1>: NumExpr,
    LT<L, NumRet<AddExp<R, U1>>>: BoolExpr,
{
    type Ret = BoolRet<LT<L, NumRet<AddExp<R, U1>>>>;
}
#[cfg(test)]
#[allow(clippy::used_underscore_items)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U5, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_lte() {
        const _0_LTE_0: () = _t::<LTE<U0, U0>>();
        const _1_LTE_0: () = _f::<LTE<U1, U0>>();
        const _1_LTE_1: () = _t::<LTE<U1, U1>>();
        const _2_LTE_1: () = _f::<LTE<U2, U1>>();
        const _1_LTE_2: () = _t::<LTE<U1, U2>>();
        const _3_LTE_1: () = _f::<LTE<U3, U1>>();
        const _4_LTE_1: () = _f::<LTE<U4, U1>>();
        const _5_LTE_6: () = _t::<LTE<U5, U6>>();
        const _1_LTE_3: () = _t::<LTE<U1, U3>>();
        const _1_LTE_4: () = _t::<LTE<U1, U4>>();
        const _2_LTE_2: () = _t::<LTE<U2, U2>>();
        const _3_LTE_3: () = _t::<LTE<U3, U3>>();
        const _6_LTE_1: () = _f::<LTE<U6, U1>>();
        const _7_LTE_1: () = _f::<LTE<U7, U1>>();
    }
    #[test]
    fn eval_gte() {
        const _0_GTE_0: () = _t::<GTE<U0, U0>>();
        const _1_GTE_0: () = _t::<GTE<U1, U0>>();
        const _1_GTE_1: () = _t::<GTE<U1, U1>>();
        const _2_GTE_1: () = _t::<GTE<U2, U1>>();
        const _2_GTE_4: () = _f::<GTE<U2, U4>>();
        const _1_GTE_2: () = _f::<GTE<U1, U2>>();
        const _3_GTE_1: () = _t::<GTE<U3, U1>>();
        const _4_GTE_1: () = _t::<GTE<U4, U1>>();
        const _4_GTE_2: () = _t::<GTE<U4, U2>>();
        const _5_GTE_6: () = _f::<GTE<U5, U6>>();
        const _1_GTE_3: () = _f::<GTE<U1, U3>>();
        const _1_GTE_4: () = _f::<GTE<U1, U4>>();
        const _2_GTE_2: () = _t::<GTE<U2, U2>>();
        const _3_GTE_3: () = _t::<GTE<U3, U3>>();
        const _6_GTE_1: () = _t::<GTE<U6, U1>>();
        const _7_GTE_1: () = _t::<GTE<U7, U1>>();
    }
}
