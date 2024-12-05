use crate::{Eval, Formula, UInt, B0, B1};

use super::Add;

impl<LB, RB> Formula for Add<UInt<LB, B0>, UInt<RB, B0>, Eval>
where
    Add<LB, RB>: Formula,
{
    type FOutput = UInt<<Add<LB, RB> as Formula>::FOutput, B0>;
}
impl<LB, RB> Formula for Add<UInt<LB, B0>, UInt<RB, B1>, Eval>
where
    Add<LB, RB, Eval>: Formula,
{
    type FOutput = UInt<<Add<LB, RB, Eval> as Formula>::FOutput, B1>;
}
impl<LB, RB> Formula for Add<UInt<LB, B1>, UInt<RB, B0>, Eval>
where
    Add<LB, RB, Eval>: Formula,
{
    type FOutput = UInt<<Add<LB, RB, Eval> as Formula>::FOutput, B1>;
}
impl<LB, RB> Formula for Add<UInt<LB, B1>, UInt<RB, B1>, Eval>
where
    Add<LB, RB, Eval>: Formula,
{
    type FOutput = UInt<<Add<LB, RB, Eval> as Formula>::FOutput, B0>;
}
