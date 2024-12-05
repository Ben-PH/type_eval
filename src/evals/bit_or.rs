use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode, B0, B1};

pub struct BitOr<L, R, M: Mode = Ast> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}

impl<L, R> Formula for BitOr<L, R>
where
    L: Formula,
    R: Formula,
    BitOr<L::FOutput, R::FOutput, Eval>: Formula,
{
    type FOutput = <BitOr<L::FOutput, R::FOutput, Eval> as Formula>::FOutput;
}

impl Formula for BitOr<B0, B0> {
    type FOutput = B0;
}
impl Formula for BitOr<B0, B1> {
    type FOutput = B1;
}
impl Formula for BitOr<B1, B0> {
    type FOutput = B1;
}
impl Formula for BitOr<B1, B1> {
    type FOutput = B1;
}
