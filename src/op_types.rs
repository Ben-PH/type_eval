use core::marker::PhantomData;

use crate::{Ast, Bit, Mode, Unsigned};

pub struct BitAnd<L: Bit, R: Bit, M: Mode = Ast> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct BitOr<L: Bit, R: Bit, M: Mode = Ast> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}

pub struct BitNot<B: Bit, M: Mode = Ast> {
    _b: PhantomData<B>,
    _m: PhantomData<M>,
}

pub struct Add<L: Unsigned, R: Unsigned, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}
pub struct Sub<L: Unsigned, R: Unsigned, M: Mode = Ast> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
    _mode: PhantomData<M>,
}

pub struct ShL<U: Unsigned, M: Mode = Ast> {
    _tp: PhantomData<U>,
    _mode: PhantomData<M>,
}
pub struct ShR<U: Unsigned, M: Mode = Ast> {
    _tp: PhantomData<U>,
    _mode: PhantomData<M>,
}

