use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode, UInt, B0, B1};


pub struct Mul<L, R, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}
pub struct MulAcc<Acc, L, R, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _acc: PhantomData<Acc>,
    _mode: PhantomData<M>,
}

impl<Acc: Formula, R: Formula> Formula for MulAcc<Acc, B0, R>
{
    type FOutput = Acc;
}
impl<L: Formula, R: Formula> Formula for Mul<L, R>
where
    MulAcc<B0, L::FOutput, R::FOutput, Eval>: Formula,
{
    type FOutput =<MulAcc<B0, L::FOutput, R::FOutput, Eval> as Formula>::FOutput;
}
impl<L: Formula, R: Formula> Formula for Mul<L, R>
where
    Mul<L::FOutput, R::FOutput, Eval>: Formula,
{
    type FOutput =<Mul<L::FOutput, R::FOutput, Eval> as Formula>::FOutput;
}
impl<L> Formula for Mul<L, B0, Eval> {
    type FOutput = B0;
}
impl Formula for Mul<B0, B1, Eval> {
    type FOutput = B1;
}
impl Formula for Mul<B1, B1, Eval> {
    type FOutput = B1;
}
