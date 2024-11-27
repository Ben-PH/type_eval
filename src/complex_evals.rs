use crate::{op_types::Add, Eval, Formula, UInt, Unsigned, B0, B1};


impl<L> Formula for Add<UInt<L, B1>, B1, Eval>
where
    L: Unsigned,
    UInt<L, B1>: Formula + Unsigned,
    Add<L, B1>: Formula + Unsigned,
    UInt<Add<L, B1>, B0>: Formula + Unsigned,
{
    type Output = UInt<Add<L, B1>, B0>;
}

