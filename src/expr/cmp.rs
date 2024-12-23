mod andor;
mod eq;
mod gt;
mod lt;
mod ltegte;

#[cfg(test)]
mod test {
    use crate::ctrl_types::{False, True, AND, GT, LT, OR};
    use crate::val_types::{_0, _1};
    use crate::{test_res::*, U2, U3};
    #[test]
    fn eval_and() {
        const _T_AND_T: () = _t::<AND<True, True>>();
        const _T_AND_F: () = _f::<AND<True, False>>();
        const _F_AND_T: () = _f::<AND<False, True>>();
        const _F_AND_F: () = _f::<AND<False, False>>();
        const _T_AND_T__AND_F: () = _f::<AND<AND<True, True>, False>>();
        const _T_AND_F__AND_T: () = _f::<AND<AND<True, False>, True>>();
    }
    #[test]
    fn eval_or() {
        const _T_OR_T: () = _t::<OR<True, True>>();
        const _T_OR_F: () = _t::<OR<True, False>>();
        const _F_OR_T: () = _t::<OR<False, True>>();
        const _F_OR_F: () = _f::<OR<False, False>>();
        const _T_OR_T__OR_F: () = _t::<OR<OR<False, True>, False>>();
        const _T_OR_F__OR_T: () = _t::<OR<OR<False, False>, True>>();
    }
    #[test]
    fn eval_cmp() {
        const _0_LT_0: () = _f::<LT<_0, _0>>();
        const _0_LT_1: () = _t::<LT<_0, _1>>();
        const _1_LT_0: () = _f::<LT<_1, _0>>();
        const _1_LT_1: () = _f::<LT<_1, _1>>();

        const _0_GT_0: () = _f::<GT<_0, _0>>();
        const _0_GT_1: () = _f::<GT<_0, _1>>();
        const _1_GT_0: () = _t::<GT<_1, _0>>();
        const _1_GT_1: () = _f::<GT<_1, _1>>();

        const _2_GT_1: () = _t::<GT<U2, _1>>();
        const _2_LT_1: () = _f::<LT<U2, _1>>();
        const _1_GT_2: () = _f::<GT<_1, U2>>();
        const _1_LT_2: () = _t::<LT<_1, U2>>();

        const _3_GT_2: () = _t::<GT<U3, U2>>();
        const _3_LT_2: () = _f::<LT<U3, U2>>();
        const _2_GT_3: () = _f::<GT<U2, U3>>();
        const _2_LT_3: () = _t::<LT<U2, U3>>();
    }
}
