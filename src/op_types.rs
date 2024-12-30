use core::marker::PhantomData;

use crate::_inners::_Recurse;

pub struct AddExp<Lhs, Rhs> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
}
pub struct SubExp<Lhs, Rhs> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
}
pub struct MulExp<Lhs, Rhs, M = _Recurse> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
    _m: PhantomData<M>,
}
pub struct DivExp<Lhs, Rhs, Acc = _Recurse> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
    _m: PhantomData<Acc>,
}
pub struct ModExp<N, D, NltD = _Recurse> {
    _n: PhantomData<N>,
    _d: PhantomData<D>,
    _cmp: PhantomData<NltD>,
}
pub struct GcdExp<Lhs, Rhs, Acc = _Recurse> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
    _m: PhantomData<Acc>,
}
pub struct ShLExp<Bs, N, M = _Recurse> {
    _bits: PhantomData<Bs>,
    _shift_count: PhantomData<N>,
    _mode: PhantomData<M>,
}
pub struct ShRExp<Bs, N, M = _Recurse> {
    _bits: PhantomData<Bs>,
    _shift_count: PhantomData<N>,
    _mode: PhantomData<M>,
}
/// Gets the index of the most significant bit
#[allow(clippy::upper_case_acronyms)]
pub struct MSB<Bs, M = _Recurse> {
    _bits: PhantomData<Bs>,
    _mode: PhantomData<M>,
}
/// Gets the index of the most significant bit
#[allow(clippy::upper_case_acronyms)]
pub struct LSB<Bs, M = _Recurse> {
    _bits: PhantomData<Bs>,
    _mode: PhantomData<M>,
}
