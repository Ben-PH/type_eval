use core::marker::PhantomData;

use crate::{Ast, Eval, Formula, Mode, Unsigned};

pub struct BitAnd<L: Formula, R: Formula, M: Mode = Ast> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}
pub struct BitOr<L: Formula, R: Formula, M: Mode = Ast> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
    _m: PhantomData<M>,
}

pub struct BitAdd<L: Formula, R: Formula, M: Mode = Ast> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
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

