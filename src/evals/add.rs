//! basics operation outputs such as B1 + B0 and visa versa, boolean algebras, etc

use crate::{
    op_types::{Add, BitOr}, Bit, BitNot, Eval, Formula, UInt, Unsigned, B0, B1
};
impl<B: Bit + Formula> Formula for BitOr<B, B1, Eval> {
    type Output = B1;
}
impl Formula for BitOr<B1, B0, Eval> {
    type Output = B1;
}
impl Formula for Add<B0, B1, Eval> {
    type Output = B1;
}
impl Formula for Add<B1, B1, Eval> {
    type Output = UInt<B1, B0>;
}

impl<L> Formula for Add<L, B0, Eval>
where
    L: Unsigned + Formula,
{
    type Output = L;
}

impl<L> Formula for Add<UInt<L, B0>, B1, Eval>
where
    L: Unsigned,
    UInt<L, B0>: Unsigned,
    UInt<L, B1>: Formula + Unsigned,
{
    type Output = UInt<L, B1>;
}
#[cfg(test)]
mod test {
    use crate::{Eval, Formula, U1, U2, U3, U4};

    use super::Add;

    #[test]
    fn compile_basic_add() {
        const fn _eval_2<F: Formula<Output = U2>>() {}
        const _ADD: () = _eval_2::<Add<U1, U1>>();

        const fn _eval_4<F: Formula<Output = U4>>() {}
        const _ADD_ADD_4: () = _eval_4::<Add<U3, U1, Eval>>();
    }
}
