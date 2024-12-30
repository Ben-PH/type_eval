use crate::{
    prelude::{False, ModExp, SubExp, True, B, LT, _0, _1},
    BoolExpr, BoolRet, NumExpr, NumRet,
};

impl<Lhs, Rhs> NumExpr for ModExp<Lhs, Rhs>
where
    Lhs: NumExpr,
    Rhs: NumExpr,
    LT<Lhs::Ret, Rhs::Ret>: BoolExpr,
    ModExp<Lhs, Rhs, BoolRet<LT<Lhs::Ret, Rhs::Ret>>>: NumExpr,
{
    type Ret = NumRet<ModExp<Lhs, Rhs, BoolRet<LT<Lhs::Ret, Rhs::Ret>>>>;
}
impl<Lhs, Lt> NumExpr for ModExp<Lhs, _1, Lt>
where
    Lhs: NumExpr,
    Lt: BoolExpr,
{
    type Ret = _0;
}
impl<Lhs, RBs, RBt> NumExpr for ModExp<Lhs, B<RBs, RBt>, True>
where
    Lhs: NumExpr,
{
    type Ret = NumRet<Lhs>;
}

impl<Lhs, RBs, RBt> NumExpr for ModExp<Lhs, B<RBs, RBt>, False>
where
    B<RBs, RBt>: NumExpr,
    SubExp<Lhs, B<RBs, RBt>>: NumExpr,
    LT<SubExp<Lhs, B<RBs, RBt>>, B<RBs, RBt>>: BoolExpr,
    ModExp<
        SubExp<Lhs, B<RBs, RBt>>,
        B<RBs, RBt>,
        BoolRet<LT<SubExp<Lhs, B<RBs, RBt>>, B<RBs, RBt>>>,
    >: NumExpr,
{
    type Ret = NumRet<
        ModExp<
            SubExp<Lhs, B<RBs, RBt>>,
            B<RBs, RBt>,
            BoolRet<LT<SubExp<Lhs, B<RBs, RBt>>, B<RBs, RBt>>>,
        >,
    >;
}

#[cfg(test)]
#[allow(clippy::used_underscore_items)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U5, U6, U7, U8},
        test_res::*,
    };
    #[test]
    fn eval_mod() {
        const _0_MOD_1: () = _b0::<ModExp<U0, U1>>();
        const _2_MOD_1: () = _b0::<ModExp<U2, U1>>();
        const _1_MOD_2: () = _b1::<ModExp<U1, U2>>();
        const _3_MOD_1: () = _b0::<ModExp<U3, U1>>();
        const _1_MOD_3: () = _b1::<ModExp<U1, U3>>();
        const _4_MOD_1: () = _b0::<ModExp<U4, U1>>();
        const _5_MOD_1: () = _b0::<ModExp<U5, U1>>();
        const _5_MOD_2: () = _b1::<ModExp<U5, U2>>();
        const _6_MOD_1: () = _b0::<ModExp<U6, U1>>();
        const _7_MOD_1: () = _b0::<ModExp<U7, U1>>();
        const _7_MOD_3: () = _b1::<ModExp<U7, U3>>();
        const _7_MOD_5: () = _b2::<ModExp<U7, U5>>();
        const _8_MOD_1: () = _b0::<ModExp<U8, U1>>();
        const _8_MOD_2: () = _b0::<ModExp<U8, U2>>();
        const _8_MOD_3: () = _b2::<ModExp<U8, U3>>();
        const _3_MOD_3: () = _b0::<ModExp<U3, U3>>();
    }
}
