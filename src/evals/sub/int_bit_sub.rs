use crate::{Eval, Formula, UInt, B0, B1};

use super::Sub;
impl Formula for Sub<UInt<B1, B0>, B1, Eval> {
    type FOutput = B1;
}
impl<L> Formula for Sub<UInt<L, B1>, B1, Eval> {
    type FOutput = UInt<L, B0>;
}
impl<L, R> Formula for Sub<UInt<L, R>, B0, Eval> {
    type FOutput = UInt<L, R>;
}
