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
    type Output = Acc;
}
impl<L: Formula, R: Formula> Formula for Mul<L, R>
where
    MulAcc<B0, L::Output, R::Output, Eval>: Formula,
{
    type Output =<MulAcc<B0, L::Output, R::Output, Eval> as Formula>::Output;
}
impl<L: Formula, R: Formula> Formula for Mul<L, R>
where
    Mul<L::Output, R::Output, Eval>: Formula,
{
    type Output =<Mul<L::Output, R::Output, Eval> as Formula>::Output;
}
impl<L> Formula for Mul<L, B0, Eval> {
    type Output = B0;
}
impl Formula for Mul<B0, B1, Eval> {
    type Output = B1;
}
impl Formula for Mul<B1, B1, Eval> {
    type Output = B1;
}
