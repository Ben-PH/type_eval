use core::marker::PhantomData;

use crate::{
    num_vals::{U0, U1},
    op_types::{AddExp, LSB, MSB},
    prelude::B,
    val_types::{NumberVal, _0, _1},
    NumExpr, NumRet,
    _inners::{_BitLit, _BitStrLit},
};

impl<BS> NumExpr for MSB<BS>
where
    BS: NumExpr,
    BS::Ret: NumberVal,
    MSBCount<U0, BS::Ret>: NumExpr,
{
    type Ret = NumRet<MSBCount<U0, BS::Ret>>;
}

#[allow(clippy::upper_case_acronyms)]
pub struct MSBCount<Idx: NumberVal, BS: NumberVal> {
    _bits: PhantomData<BS>,
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
impl<Idx, BH, BT> NumExpr for MSBCount<Idx, B<BH, BT>>
where
    Idx: NumberVal,
    BH: NumberVal + _BitStrLit,
    BT: _BitLit,
    AddExp<Idx, U1>: NumExpr,
    NumRet<AddExp<Idx, U1>>: NumberVal,
    MSBCount<NumRet<AddExp<Idx, U1>>, BH>: NumExpr,
    NumRet<MSBCount<NumRet<AddExp<Idx, U1>>, BH>>: NumberVal,
{
    type Ret = NumRet<MSBCount<NumRet<AddExp<Idx, U1>>, BH>>;
}

impl<BS> NumExpr for LSB<BS>
where
    BS: NumExpr,
    BS::Ret: NumberVal,
    LSBCount<U0, BS::Ret>: NumExpr,
{
    type Ret = NumRet<LSBCount<U0, BS::Ret>>;
}

#[allow(clippy::upper_case_acronyms)]
pub struct LSBCount<Idx: NumberVal, BS: NumberVal> {
    _bits: PhantomData<BS>,
    _idx: PhantomData<Idx>,
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
impl<Idx, BH> NumExpr for LSBCount<Idx, B<BH, _0>>
where
    Idx: NumberVal,
    BH: NumberVal + _BitStrLit,
    AddExp<Idx, U1>: NumExpr,
    NumRet<AddExp<Idx, U1>>: NumberVal,
    LSBCount<NumRet<AddExp<Idx, U1>>, BH>: NumExpr,
{
    type Ret = NumRet<LSBCount<NumRet<AddExp<Idx, U1>>, BH>>;
}
impl<Idx, BS> NumExpr for LSBCount<Idx, B<BS, _1>>
where
    Idx: NumberVal,
    BS: _BitStrLit,
{
    type Ret = Idx;
}
#[cfg(test)]
#[allow(clippy::used_underscore_items)]
#[allow(non_upper_case_globals)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U2, U3, U5, U6},
        op_types::LSB,
        test_res::*,
    };
    #[test]
    fn eval_msb() {
        const ___0: () = _b0::<MSB<_0>>();
        const ___1: () = _b0::<MSB<_1>>();
        const __U2: () = _b1::<MSB<U2>>();
        const __10: () = _b1::<MSB<B<_1, _0>>>();
        const __U3: () = _b1::<MSB<U3>>();
        const __11: () = _b1::<MSB<B<_1, _1>>>();
        const _100: () = _b2::<MSB<B<B<_1, _0>, _0>>>();
        const _101: () = _b2::<MSB<B<B<_1, _0>, _1>>>();
        const _3ADD2: () = _b2::<MSB<AddExp<U3, U2>>>();
        const _110: () = _b2::<MSB<B<B<_1, _1>, _0>>>();
        const _111: () = _b2::<MSB<B<B<_1, _1>, _1>>>();
    }

    #[test]
    fn eval_lsb() {
        const ___0: () = _b0::<LSB<_0>>();
        const ___1: () = _b0::<LSB<_1>>();
        const __10: () = _b1::<LSB<B<_1, _0>>>();
        const __11: () = _b0::<LSB<B<_1, _1>>>();
        const _111: () = _b0::<LSB<B<B<_1, _1>, _1>>>();
        const _101: () = _b0::<LSB<B<B<_1, _0>, _1>>>();
        const _3ADD2: () = _b0::<LSB<AddExp<U3, U2>>>();
        const _110: () = _b1::<LSB<B<B<_1, _1>, _0>>>();
        const _100: () = _b2::<LSB<B<B<_1, _0>, _0>>>();
        const _1000: () = _b1::<LSB<B<B<B<_1, _0>, _1>, _0>>>();
        const _1100: () = _b2::<LSB<B<B<B<_1, _1>, _0>, _0>>>();
        const _1010: () = _b1::<LSB<B<B<B<_1, _0>, _1>, _0>>>();
    }
    #[test]
    fn eval_mlsb_nested() {
        const _MSB5_ADD_LSB6: () = _b1::<MSB<AddExp<MSB<U5>, LSB<U6>>>>();
    }
}
