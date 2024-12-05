use crate::{Eval, Formula, B0, B1};

use super::Sub;
impl Formula for Sub<B0, B0, Eval> {
    type FOutput = B0;
}
impl Formula for Sub<B1, B0, Eval> {
    type FOutput = B1;
}
impl Formula for Sub<B1, B1, Eval> {
    type FOutput = B0;
}
