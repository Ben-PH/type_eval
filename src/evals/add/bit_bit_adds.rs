use crate::{Eval, Formula, UInt, B0, B1};

use super::Add;
impl Formula for Add<B0, B0, Eval> {
    type FOutput = B0;
}
impl Formula for Add<B0, B1, Eval> {
    type FOutput = B1;
}
impl Formula for Add<B1, B0, Eval> {
    type FOutput = B1;
}
impl Formula for Add<B1, B1, Eval> {
    type FOutput = UInt<B1, B0>;
}
