use crate::{
    _inners::_Base,
    op_types::GcdExp,
    prelude::{ModExp, B, _0, _1},
    NumExpr, NumRet,
};

impl<N, D> NumExpr for GcdExp<N, D>
where
    GcdExp<N, D, _Base>: NumExpr,
{
    type Ret = NumRet<GcdExp<N, D, _Base>>;
}

impl<N> NumExpr for GcdExp<N, _1, _Base>
where
    N: NumExpr,
{
    type Ret = _1;
}
impl<N> NumExpr for GcdExp<N, _0, _Base>
where
    N: NumExpr,
{
    type Ret = N::Ret;
}
impl<N, DBs, DB> NumExpr for GcdExp<N, B<DBs, DB>, _Base>
where
    ModExp<N, B<DBs, DB>>: NumExpr,
    N: NumExpr,
    GcdExp<B<DBs, DB>, NumRet<ModExp<N, B<DBs, DB>>>, _Base>: NumExpr,
{
    type Ret = NumRet<GcdExp<B<DBs, DB>, NumRet<ModExp<N, B<DBs, DB>>>, _Base>>;
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
    fn eval_gcd() {
        const _0_GCD_1: () = _b1::<GcdExp<U0, U1>>();
        const _2_GCD_1: () = _b1::<GcdExp<U2, U1>>();
        const _1_GCD_2: () = _b1::<GcdExp<U1, U2>>();
        const _3_GCD_1: () = _b1::<GcdExp<U3, U1>>();
        const _1_GCD_3: () = _b1::<GcdExp<U1, U3>>();
        const _4_GCD_1: () = _b1::<GcdExp<U4, U1>>();
        const _5_GCD_1: () = _b1::<GcdExp<U5, U1>>();
        const _5_GCD_2: () = _b1::<GcdExp<U5, U2>>();
        const _6_GCD_1: () = _b1::<GcdExp<U6, U1>>();
        const _7_GCD_1: () = _b1::<GcdExp<U7, U1>>();
        const _7_GCD_3: () = _b1::<GcdExp<U7, U3>>();
        const _7_GCD_5: () = _b1::<GcdExp<U7, U5>>();
        const _8_GCD_1: () = _b1::<GcdExp<U8, U1>>();
        const _8_GCD_2: () = _b2::<GcdExp<U8, U2>>();
        const _8_GCD_3: () = _b1::<GcdExp<U8, U3>>();
        const _3_GCD_3: () = _b3::<GcdExp<U3, U3>>();
    }
}
