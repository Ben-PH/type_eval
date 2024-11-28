use crate::{BitNot, Eval, Formula, B0, B1};


impl Formula for BitNot<B0, Eval> {
    type Output = B1;
}

impl Formula for BitNot<B1, Eval> {
    type Output = B0;
}

