//! Defines the manner in which expressions can recurse towards a base-case

mod add;
mod cmp;
mod mlsb;
mod mul;
mod shlr;
mod sub;

#[cfg(test)]
mod test {
    use crate::{
        op_types::{Add, ShL, Sub, MSB},
        test_res::*,
        U0, U1, U2, U3, U4, U6,
    };

    #[test]
    #[allow(non_upper_case_globals)]
    fn add_sub() {
        // const _2_ADD_1_SUB_3: () = _b3::<Add<BitString<_1, _0>, _1>>();
        const _2_ADD_1__SUB_3: () = _b0::<Sub<Add<U2, U1>, U3>>();
        const _6_SUB__1_ADD_3: () = _b2::<Sub<U6, Add<U1, U3>>>();
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn shift_msb() {
        // const _2_ADD_1_SUB_3: () = _b3::<Add<BitString<_1, _0>, _1>>();
        const _MSB__2_SHL_1: () = _b2::<MSB<ShL<U2, U1>>>();
        const _MSB__2_SHL_0: () = _b1::<MSB<ShL<U2, U0>>>();

        const _MSB_4__SUB__MSB_3: () = _b1::<Sub<MSB<U4>, MSB<U3>>>();
        const _MSB_4__ADD__MSB_3: () = _b3::<Add<MSB<U4>, MSB<U3>>>();
    }
}
