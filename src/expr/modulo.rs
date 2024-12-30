use crate::{
    prelude::{False, ModExp, SubExp, True, B, LT, _0, _1},
    BoolExpr, BoolRet, NumExpr, NumRet,
};

impl<N, D> NumExpr for ModExp<N, D>
where
    N: NumExpr,
    D: NumExpr,
    LT<N::Ret, D::Ret>: BoolExpr,
    ModExp<N, D, BoolRet<LT<N::Ret, D::Ret>>>: NumExpr,
{
    type Ret = NumRet<ModExp<N, D, BoolRet<LT<N::Ret, D::Ret>>>>;
}
impl<N, Lt> NumExpr for ModExp<N, _1, Lt>
where
    N: NumExpr,
    Lt: BoolExpr,
{
    type Ret = _0;
}
impl<N, DBs, DBt> NumExpr for ModExp<N, B<DBs, DBt>, True>
where
    N: NumExpr,
{
    type Ret = NumRet<N>;
}

impl<N, DBs, DBt> NumExpr for ModExp<N, B<DBs, DBt>, False>
where
    B<DBs, DBt>: NumExpr,
    SubExp<N, B<DBs, DBt>>: NumExpr,
    LT<SubExp<N, B<DBs, DBt>>, B<DBs, DBt>>: BoolExpr,
    ModExp<SubExp<N, B<DBs, DBt>>, B<DBs, DBt>, BoolRet<LT<SubExp<N, B<DBs, DBt>>, B<DBs, DBt>>>>:
        NumExpr,
{
    type Ret = NumRet<
        ModExp<
            SubExp<N, B<DBs, DBt>>,
            B<DBs, DBt>,
            BoolRet<LT<SubExp<N, B<DBs, DBt>>, B<DBs, DBt>>>,
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
