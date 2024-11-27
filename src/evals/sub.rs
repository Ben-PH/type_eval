use crate::{op_types::Sub, Eval, Formula, UInt, Unsigned, B0, B1};



impl<L> Formula for Sub<L, B0, Eval>
where
    L: Unsigned + Formula,
{
    type Output = L;
}
impl<L> Formula for Sub<UInt<L, B1>, B1, Eval>
where
    L: Unsigned,
{
    type Output = UInt<L, B0>;
}

