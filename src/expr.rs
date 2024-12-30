//! Defines the manner in which expressions can recurse towards a base-case

mod add;
mod cmp;
mod div;
mod gcd;
mod if_branch;
mod lcm;
mod mlsb;
mod modulo;
mod mul;
mod shlr;
mod sub;

#[cfg(test)]
#[allow(clippy::used_underscore_items)]
mod test {
    use crate::{prelude::*, test_res::*, BoolExpr, NumExpr};

    #[test]
    #[allow(non_upper_case_globals)]
    fn add_sub() {
        const _2_ADD_1__SUB_3: () = _b0::<SubExp<AddExp<U2, U1>, U3>>();
        const _6_SUB__1_ADD_3: () = _b2::<SubExp<U6, AddExp<U1, U3>>>();
        // const COMPILE_FAIL: () = _b2::<SubExp<U7, AddExp<U1, U3>>>();
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn shift_msb() {
        const _MSB__2_SHL_1: () = _b2::<MSB<ShLExp<U2, U1>>>();
        const _MSB__2_SHL_0: () = _b1::<MSB<ShLExp<U2, U0>>>();

        const _MSB_4__SUB__MSB_3: () = _b1::<SubExp<MSB<U4>, MSB<U3>>>();
        const _MSB_4__ADD__MSB_3: () = _b3::<AddExp<MSB<U4>, MSB<U3>>>();
    }
    #[test]
    #[allow(non_upper_case_globals)]
    fn ifs() {
        // if (1/2) < 1 {0/1} else {1/0}
        const _IF_T_U0_DIV0: () =
            _b0::<IF<LT<DivExp<U1, U2>, U1>, DivExp<U0, U1>, DivExp<U1, U0>>>();
        // div-by-zero is a compile fail
        // const _COMPILE_FAIL: () = _b0::<DivExp<U1, U0>>();
    }
    #[test]
    #[allow(non_upper_case_globals)]
    fn lt4() {
        // define a function that carries a proof of evaluaton result < 6
        const fn _lt6<E: NumExpr>()
        where
            LT<E::Ret, U6>: BoolExpr<Ret = True>,
        {
        }

        // 2 + 3 < 6
        const _5_LT_6: () = _lt6::<AddExp<U2, U3>>();
        // 2 * 3 !< 6
        // const FAIL: () = _lt6::<MulExp<U2, U3>>();
    }
}
