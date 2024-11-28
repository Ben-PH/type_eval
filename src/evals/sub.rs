use crate::{op_types::Sub, Formula, UInt, B0, B1};

impl<L> Formula for Sub<L, B0>
where
    L: Formula,
{
    type Output = L;
}
impl<L> Formula for Sub<UInt<L, B1>, B1>
where
    UInt<L, B0>: Formula,
{
    type Output = UInt<L, B0>;
}
