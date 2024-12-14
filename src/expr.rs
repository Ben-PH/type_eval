//! Defines the manner in which expressions can recurse towards a base-case

/// Addition expressions
mod add;
mod mul;
/// Subtraction expressions
mod sub;

#[cfg(test)]
mod test {
    use crate::{
        op_types::{Add, Sub},
        test_res::*,
        val_types::{BitString, _0, _1},
    };

    #[test]
    fn add_sub() {
        // const _2_ADD_1_SUB_3: () = _b3::<Add<BitString<_1, _0>, _1>>();
        const _2_ADD_1__SUB_3: () = _b0::<Sub<Add<BitString<_1, _0>, _1>, BitString<_1, _1>>>();
        const _6_SUB__1_ADD_3: () =
            _b2::<Sub<BitString<BitString<_1, _1>, _0>, Add<_1, BitString<_1, _1>>>>();
    }
}
