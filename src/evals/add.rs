//! basics operation outputs such as B1 + B0 and visa versa, boolean algebras, etc

use crate::{
    op_types::{Add, BitAdd, BitOr}, Bit, Eval, Formula, UInt, Unsigned, B0, B1
};
impl<B: Bit + Formula> Formula for BitOr<B, B1, Eval> {
    type Output = B1;
}
impl Formula for BitOr<B1, B0, Eval> {
    type Output = B1;
}

impl<BL: Bit + Formula + Unsigned> Formula for BitAdd<BL, B0, Eval> {
    type Output = BL;
}
impl Formula for BitAdd<B0, B1, Eval> {
    type Output = B1;
}
impl Formula for BitAdd<B1, B1, Eval> {
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
