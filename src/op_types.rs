use core::marker::PhantomData;

use crate::Recurse;

pub struct AddExp<Lhs, Rhs> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
}
pub struct SubExp<Lhs, Rhs> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
}
pub struct MulExp<Lhs, Rhs, M = Recurse> {
    _l: PhantomData<Lhs>,
    _r: PhantomData<Rhs>,
    _m: PhantomData<M>,
}
pub struct ShLExp<Bs, N, M = Recurse> {
    _bits: PhantomData<Bs>,
    _shift_count: PhantomData<N>,
    _mode: PhantomData<M>,
}
pub struct ShRExp<Bs, N, M = Recurse> {
    _bits: PhantomData<Bs>,
    _shift_count: PhantomData<N>,
    _mode: PhantomData<M>,
}
/// Gets the index of the most significant bit
#[allow(clippy::upper_case_acronyms)]
pub struct MSB<Bs, M = Recurse> {
    _bits: PhantomData<Bs>,
    _mode: PhantomData<M>,
}
/// Gets the index of the most significant bit
#[allow(clippy::upper_case_acronyms)]
pub struct LSB<Bs, M = Recurse> {
    _bits: PhantomData<Bs>,
    _mode: PhantomData<M>,
}
