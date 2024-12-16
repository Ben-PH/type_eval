use core::marker::PhantomData;

use crate::{
    op_types::{Add, MSB},
    val_types::{BitLit, BitStrLit, BitString, NumberVal, _0, _1},
    NumExpr, NumRet,
};

impl<BS> NumExpr for MSB<BS>
where
    BS: NumExpr,
    MSBCount<_0, BS::Ret>: NumExpr,
{
    type Ret = NumRet<MSBCount<_0, BS::Ret>>;
}

#[allow(clippy::upper_case_acronyms)]
pub struct MSBCount<Idx: NumberVal, Bs: NumberVal> {
    _bits: PhantomData<Bs>,
    _idx: PhantomData<Idx>,
    // _mode: PhantomData<M>,
}

impl<Idx> NumExpr for MSBCount<Idx, _0>
where
    Idx: NumberVal,
{
    type Ret = Idx;
}
impl<Idx> NumExpr for MSBCount<Idx, _1>
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
    Add<Idx, _1>: NumExpr,
    MSBCount<NumRet<Add<Idx, _1>>, BS>: NumExpr,
{
    type Ret = NumRet<MSBCount<NumRet<Add<Idx, _1>>, BS>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_res::*;
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
}
