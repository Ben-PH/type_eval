use crate::{_inners::_Base, ctrl_types::LT, prelude::GT, BoolExpr, BoolRet, NumExpr};

impl<L, R> BoolExpr for LT<L, R>
where
    L: NumExpr,
    R: NumExpr,
    GT<R::Ret, L::Ret, _Base>: BoolExpr,
{
    type Ret = BoolRet<GT<R::Ret, L::Ret, _Base>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U5, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_lt() {
        const _0_LT_0: () = _f::<LT<U0, U0>>();
        const _1_LT_0: () = _f::<LT<U1, U0>>();
        const _1_LT_1: () = _f::<LT<U1, U1>>();
        const _2_LT_1: () = _f::<LT<U2, U1>>();
        const _1_LT_2: () = _t::<LT<U1, U2>>();
        const _3_LT_1: () = _f::<LT<U3, U1>>();
        const _4_LT_1: () = _f::<LT<U4, U1>>();
        const _4_LT_2: () = _f::<LT<U4, U2>>();
        const _5_LT_6: () = _t::<LT<U5, U6>>();
        const _1_LT_3: () = _t::<LT<U1, U3>>();
        const _1_LT_4: () = _t::<LT<U1, U4>>();
        const _2_LT_2: () = _f::<LT<U2, U2>>();
        const _3_LT_3: () = _f::<LT<U3, U3>>();
        const _6_LT_1: () = _f::<LT<U6, U1>>();
        const _7_LT_1: () = _f::<LT<U7, U1>>();
    }
}
