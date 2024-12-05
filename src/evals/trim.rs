use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode, UInt, B0};


pub(crate) struct Trim<F, M: Mode = Ast> {
    _form: PhantomData<F>,
    _mode: PhantomData<M>,
}
impl<L: Formula> Formula for Trim<UInt<B0, L>, Eval> {
    type FOutput = L::FOutput;
}

