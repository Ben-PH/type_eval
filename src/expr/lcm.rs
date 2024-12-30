use crate::{
    _inners::_Base,
    op_types::LcmExp,
    prelude::{DivExp, GcdExp, ModExp, MulExp, B, _0, _1},
    NumExpr, NumRet,
};

impl<Lhs, R> NumExpr for LcmExp<Lhs, R>
where
    GcdExp<Lhs, R>: NumExpr,
    MulExp<Lhs, R>: NumExpr,
    DivExp<NumRet<MulExp<Lhs, R>>, NumRet<GcdExp<Lhs, R>>>: NumExpr,
{
    type Ret = NumRet<DivExp<NumRet<MulExp<Lhs, R>>, NumRet<GcdExp<Lhs, R>>>>;
}

impl<Lhs> NumExpr for LcmExp<Lhs, _1, _Base>
where
    Lhs: NumExpr,
{
    type Ret = _1;
}
impl<Lhs> NumExpr for LcmExp<Lhs, _0, _Base>
where
    Lhs: NumExpr,
{
    type Ret = Lhs::Ret;
}
impl<Lhs, RBs, RB> NumExpr for LcmExp<Lhs, B<RBs, RB>, _Base>
where
    ModExp<Lhs, B<RBs, RB>>: NumExpr,
    Lhs: NumExpr,
    LcmExp<B<RBs, RB>, NumRet<ModExp<Lhs, B<RBs, RB>>>, _Base>: NumExpr,
{
    type Ret = NumRet<LcmExp<B<RBs, RB>, NumRet<ModExp<Lhs, B<RBs, RB>>>, _Base>>;
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
    fn eval_lcm() {
        const _0_LCM_1: () = _b0::<LcmExp<U0, U1>>();
        const _2_LCM_1: () = _b2::<LcmExp<U2, U1>>();
        const _1_LCM_2: () = _b2::<LcmExp<U1, U2>>();
        const _3_LCM_1: () = _b3::<LcmExp<U3, U1>>();
        const _1_LCM_3: () = _b3::<LcmExp<U1, U3>>();
        const _4_LCM_1: () = _b4::<LcmExp<U4, U1>>();
        const _5_LCM_2: () = _b10::<LcmExp<U5, U2>>();
        const _6_LCM_1: () = _b6::<LcmExp<U6, U1>>();
        const _7_LCM_1: () = _b7::<LcmExp<U7, U1>>();
        const _7_LCM_3: () = _b21::<LcmExp<U7, U3>>();
        const _7_LCM_5: () = _b35::<LcmExp<U7, U5>>();
        const _3_LCM_3: () = _b3::<LcmExp<U3, U3>>();
    }
}
