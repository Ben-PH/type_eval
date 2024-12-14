use core::marker::PhantomData;

use crate::Recurse;

pub struct Add<Lhs, Rhs, M = Recurse> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
    _m: PhantomData<M>,
}
pub struct Sub<Lhs, Rhs, M = Recurse> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
    _m: PhantomData<M>,
}
pub struct Mul<Lhs, Rhs, M = Recurse> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
    _m: PhantomData<M>,
}
// pub struct BitAnd<L, R, M: Mode = Ast> {
//     _l: PhantomData<L>,
//     _r: PhantomData<R>,
//     _m: PhantomData<M>,
// }
// pub struct BitNot<B, M: Mode = Ast> {
//     _b: PhantomData<B>,
//     _m: PhantomData<M>,
// }
//
// pub struct ShL<U, M: Mode = Ast> {
//     _tp: PhantomData<U>,
//     _mode: PhantomData<M>,
// }
// pub struct ShR<U, M: Mode = Ast> {
//     _tp: PhantomData<U>,
//     _mode: PhantomData<M>,
// }
