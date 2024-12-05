use crate::{Eval, Formula, UInt, B0, B1};

use super::Sub;
impl<LB, RB> Formula for Sub<UInt<LB, B0>, UInt<RB, B0>, Eval>
where
    Sub<LB, RB>: Formula,
{
    type FOutput = UInt<<Sub<LB, RB> as Formula>::FOutput, B0>;
}
impl<LB, RB> Formula for Sub<UInt<LB, B1>, UInt<RB, B0>, Eval>
where
    Sub<LB, RB>: Formula,
{
    type FOutput = UInt<<Sub<LB, RB> as Formula>::FOutput, B1>;
}
impl<LB, RB> Formula for Sub<UInt<LB, B1>, UInt<RB, B1>, Eval>
where
    Sub<LB, RB>: Formula,
{
    type FOutput = UInt<<Sub<LB, RB> as Formula>::FOutput, B0>;
}
