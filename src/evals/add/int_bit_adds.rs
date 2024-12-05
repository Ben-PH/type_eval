use crate::{Eval, Formula, UInt, B0, B1};

use super::Add;
impl Formula for Add<UInt<B1, B0>, B1, Eval> {
    type FOutput = UInt<B1, B1>;
}
impl Formula for Add<UInt<B1, B1>, B1, Eval> {
    type FOutput = UInt<UInt<B1, B0>, B0>;
}
impl<L, R> Formula for Add<UInt<UInt<L, R>, B0>, B1, Eval> {
    type FOutput = UInt<UInt<L, R>, B1>;
}
impl<L, R> Formula for Add<UInt<UInt<L, R>, B1>, B1, Eval> {
    type FOutput = UInt<Add<UInt<L, R>, B1>, B0>;
}

impl<L, R> Formula for Add<B1, UInt<L, R>, Eval>
where
    Add<UInt<L, R>, B1>: Formula,
{
    type FOutput = <Add<UInt<L, R>, B1> as Formula>::FOutput;
}

impl<L, R> Formula for Add<B0, UInt<L, R>, Eval>
where
    Add<UInt<L, R>, B0>: Formula,
{
    type FOutput = <Add<UInt<L, R>, B0> as Formula>::FOutput;
}
