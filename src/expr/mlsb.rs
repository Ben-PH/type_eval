use core::marker::PhantomData;

use crate::{
    num_vals::{U0, U1},
    op_types::{AddExp, LSB, MSB},
    val_types::{BitLit, BitStrLit, NumberVal, _0, _1},
    BitString, NumExpr, NumRet,
};

impl<BS> NumExpr for MSB<BS>
where
    BS: NumExpr,
    MSBCount<U0, BS::Ret>: NumExpr,
{
    type Ret = NumRet<MSBCount<U0, BS::Ret>>;
}

#[allow(clippy::upper_case_acronyms)]
pub struct MSBCount<Idx: NumberVal, Bs: NumberVal> {
    _bits: PhantomData<Bs>,
    _idx: PhantomData<Idx>,
    // _mode: PhantomData<M>,
}

impl<Idx> NumExpr for MSBCount<Idx, U0>
where
    Idx: NumberVal,
{
    type Ret = Idx;
}
impl<Idx> NumExpr for MSBCount<Idx, U1>
where
    Idx: NumberVal,
{
    type Ret = Idx;
}
impl<Idx, BS, B> NumExpr for MSBCount<Idx, BitString<BS, B>>
where
    Idx: NumberVal,
    BS: NumberVal + BitStrLit,
    B: BitLit,
    AddExp<Idx, U1>: NumExpr,
    MSBCount<NumRet<AddExp<Idx, U1>>, BS>: NumExpr,
{
    type Ret = NumRet<MSBCount<NumRet<AddExp<Idx, U1>>, BS>>;
}

impl<BS> NumExpr for LSB<BS>
where
    BS: NumExpr,
    LSBCount<U0, BS::Ret>: NumExpr,
{
    type Ret = NumRet<LSBCount<U0, BS::Ret>>;
}

#[allow(clippy::upper_case_acronyms)]
pub struct LSBCount<Idx: NumberVal, Bs: NumberVal> {
    _bits: PhantomData<Bs>,
    _idx: PhantomData<Idx>,
    // _mode: PhantomData<M>,
}

impl<Idx> NumExpr for LSBCount<Idx, U0>
where
    Idx: NumberVal,
{
    type Ret = Idx;
}
impl<Idx> NumExpr for LSBCount<Idx, U1>
where
    Idx: NumberVal,
{
    type Ret = Idx;
}
impl<Idx, BS> NumExpr for LSBCount<Idx, BitString<BS, _0>>
where
    Idx: NumberVal,
    BS: NumberVal + BitStrLit,
    AddExp<Idx, U1>: NumExpr,
    LSBCount<NumRet<AddExp<Idx, U1>>, BS>: NumExpr,
{
    type Ret = NumRet<LSBCount<NumRet<AddExp<Idx, U1>>, BS>>;
}
impl<Idx, BS> NumExpr for LSBCount<Idx, BitString<BS, _1>>
where
    Idx: NumberVal,
    BS: BitStrLit,
{
    type Ret = Idx;
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{op_types::LSB, test_res::*};
    #[allow(non_upper_case_globals)]
    #[test]
    fn eval_msb() {
        const ___0: () = _b0::<MSB<_0>>();
        const ___1: () = _b0::<MSB<_1>>();
        const __10: () = _b1::<MSB<BitString<_1, _0>>>();
        const __11: () = _b1::<MSB<BitString<_1, _1>>>();
        const _111: () = _b2::<MSB<BitString<BitString<_1, _1>, _1>>>();
        const _101: () = _b2::<MSB<BitString<BitString<_1, _0>, _1>>>();
        const _110: () = _b2::<MSB<BitString<BitString<_1, _1>, _0>>>();
    }
    #[allow(non_upper_case_globals)]
    #[test]
    fn eval_lsb() {
        const ___0: () = _b0::<LSB<_0>>();
        const ___1: () = _b0::<LSB<_1>>();
        const __10: () = _b1::<LSB<BitString<_1, _0>>>();
        const __11: () = _b0::<LSB<BitString<_1, _1>>>();
        const _111: () = _b0::<LSB<BitString<BitString<_1, _1>, _1>>>();
        const _101: () = _b0::<LSB<BitString<BitString<_1, _0>, _1>>>();
        const _110: () = _b1::<LSB<BitString<BitString<_1, _1>, _0>>>();
        const _100: () = _b2::<LSB<BitString<BitString<_1, _0>, _0>>>();
        const _1000: () = _b1::<LSB<BitString<BitString<BitString<_1, _0>, _1>, _0>>>();
        const _1100: () = _b2::<LSB<BitString<BitString<BitString<_1, _1>, _0>, _0>>>();
        const _1010: () = _b1::<LSB<BitString<BitString<BitString<_1, _0>, _1>, _0>>>();
    }
}
