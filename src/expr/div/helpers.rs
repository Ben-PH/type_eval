use core::marker::PhantomData;

use crate::{
    prelude::{ShLExp, SubExp, MSB},
    NumExpr, NumRet,
};

pub struct _SyncLen<L, R> {
    _lhs: PhantomData<L>,
    _rhs: PhantomData<R>,
}

impl<L, R> NumExpr for _SyncLen<L, R>
where
    L: NumExpr,
    R: NumExpr,
    MSB<L::Ret>: NumExpr,
    MSB<R::Ret>: NumExpr,
    SubExp<NumRet<MSB<L::Ret>>, NumRet<MSB<R::Ret>>>: NumExpr,
    ShLExp<R, NumRet<SubExp<NumRet<MSB<L::Ret>>, NumRet<MSB<R::Ret>>>>>: NumExpr,
{
    type Ret = NumRet<ShLExp<R, NumRet<SubExp<NumRet<MSB<L::Ret>>, NumRet<MSB<R::Ret>>>>>>;
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U5, U6, U7},
        test_res::*,
    };
    #[test]
    fn eval_len_sync() {
        const _0_SYNC_0: () = _b0::<_SyncLen<U0, U0>>();
        const _1_SYNC_0: () = _b0::<_SyncLen<U1, U0>>();
        const _1_SYNC_1: () = _b1::<_SyncLen<U1, U1>>();
        const _2_SYNC_1: () = _b2::<_SyncLen<U2, U1>>();
        const _3_SYNC_1: () = _b2::<_SyncLen<U3, U1>>();
        const _4_SYNC_1: () = _b4::<_SyncLen<U4, U1>>();
        const _6_SYNC_5: () = _b5::<_SyncLen<U6, U5>>();
        const _2_SYNC_2: () = _b2::<_SyncLen<U2, U2>>();
        const _3_SYNC_3: () = _b3::<_SyncLen<U3, U3>>();
        const _6_SYNC_1: () = _b4::<_SyncLen<U6, U1>>();
        const _7_SYNC_1: () = _b4::<_SyncLen<U7, U1>>();
    }
}
