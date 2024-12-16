//! Defines the manner in which expressions can recurse towards a base-case

mod add;
mod cmp;
mod msb;
mod mul;
mod shlr;
mod sub;

#[cfg(test)]
mod test {
    use crate::{
        op_types::{Add, ShL, Sub, MSB},
        test_res::*,
        val_types::{BitString, _0, _1},
    };

    #[test]
    #[allow(non_upper_case_globals)]
    fn add_sub() {
        // const _2_ADD_1_SUB_3: () = _b3::<Add<BitString<_1, _0>, _1>>();
        const _2_ADD_1__SUB_3: () = _b0::<Sub<Add<BitString<_1, _0>, _1>, BitString<_1, _1>>>();
        const _6_SUB__1_ADD_3: () =
            _b2::<Sub<BitString<BitString<_1, _1>, _0>, Add<_1, BitString<_1, _1>>>>();
    }

    #[test]
    #[allow(non_upper_case_globals)]
    fn shift_msb() {
        // const _2_ADD_1_SUB_3: () = _b3::<Add<BitString<_1, _0>, _1>>();
        const _MSB__2_SHL_1: () = _b2::<MSB<ShL<BitString<_1, _0>, _1>>>();
        const _MSB__2_SHL_0: () = _b1::<MSB<ShL<BitString<_1, _0>, _0>>>();

        const _MSB_4__SUB__MSB_3: () =
            _b1::<Sub<MSB<BitString<BitString<_1, _0>, _0>>, MSB<BitString<_1, _1>>>>();
        const _MSB_4__ADD__MSB_3: () =
            _b3::<Add<MSB<BitString<BitString<_1, _0>, _0>>, MSB<BitString<_1, _1>>>>();
    }
}
