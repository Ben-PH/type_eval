mod and;
mod eq;
mod gt;
mod lt;
mod ltegte;

#[cfg(test)]
mod test {
    use crate::ctrl_types::{False, True, AND, GT, LT};
    use crate::test_res::*;
    use crate::val_types::{BitString, _0, _1};
    #[test]
    fn eval_add() {
        const _T_AND_T: () = _t::<AND<True, True>>();
        const _T_AND_F: () = _f::<AND<True, False>>();
        const _F_AND_T: () = _f::<AND<False, True>>();
        const _F_AND_F: () = _f::<AND<False, False>>();
        const _T_AND_T__AND_F: () = _f::<AND<AND<True, True>, False>>();
        const _T_AND_F__AND_T: () = _f::<AND<AND<True, False>, True>>();

        const _0_LT_0: () = _f::<LT<_0, _0>>();
        const _0_LT_1: () = _t::<LT<_0, _1>>();
        const _1_LT_0: () = _f::<LT<_1, _0>>();
        const _1_LT_1: () = _f::<LT<_1, _1>>();

        const _0_GT_0: () = _f::<GT<_0, _0>>();
        const _0_GT_1: () = _f::<GT<_0, _1>>();
        const _1_GT_0: () = _t::<GT<_1, _0>>();
        const _1_GT_1: () = _f::<GT<_1, _1>>();

        const _2_GT_1: () = _t::<GT<BitString<_1, _0>, _1>>();
        const _2_LT_1: () = _f::<LT<BitString<_1, _0>, _1>>();
        const _1_GT_2: () = _f::<GT<_1, BitString<_1, _0>>>();
        const _1_LT_2: () = _t::<LT<_1, BitString<_1, _0>>>();

        const _3_GT_2: () = _t::<GT<BitString<_1, _1>, BitString<_1, _0>>>();
        const _3_LT_2: () = _f::<LT<BitString<_1, _1>, BitString<_1, _0>>>();
        const _2_GT_3: () = _f::<GT<BitString<_1, _0>, BitString<_1, _1>>>();
        const _2_LT_3: () = _t::<LT<BitString<_1, _0>, BitString<_1, _1>>>();
    }
}
