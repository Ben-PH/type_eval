use core::marker::PhantomData;

use crate::{
    _inners::{_Base, _Recurse},
    op_types::DivExp,
    prelude::{AddExp, SubExp, B, IF, LT, _0, _1},
    val_types::NumberVal,
    BoolExpr, NumExpr, NumRet,
};

impl<N, D> NumExpr for DivExp<N, D>
where
    DivExp<N, D, _Base>: NumExpr,
{
    type Ret = NumRet<DivExp<N, D, _Base>>;
}

impl<N> NumExpr for DivExp<N, _1, _Base>
where
    N: NumberVal,
{
    type Ret = N;
}
impl<DBs, DB> NumExpr for DivExp<_1, B<DBs, DB>, _Base>
where
    B<DBs, DB>: NumberVal,
{
    type Ret = _0;
}
impl<DBs, DB> NumExpr for DivExp<_0, B<DBs, DB>, _Base>
where
    B<DBs, DB>: NumberVal,
{
    type Ret = _0;
}
impl<NBs, NB, DBs, DB> NumExpr for DivExp<B<NBs, NB>, B<DBs, DB>, _Base>
where
    B<NBs, NB>: NumberVal,
    B<DBs, DB>: NumberVal,
    DivExp<B<NBs, NB>, B<DBs, DB>, _0>: NumExpr,
{
    type Ret = NumRet<DivExp<B<NBs, NB>, B<DBs, DB>, _0>>;
}
impl<DBs, DB, Acc> NumExpr for DivExp<_1, B<DBs, DB>, Acc>
where
    Acc: NumExpr,
    Acc::Ret: NumberVal,
    B<DBs, DB>: NumberVal,
{
    type Ret = Acc::Ret;
}
impl<DBs, DB, Acc> NumExpr for DivExp<_0, B<DBs, DB>, Acc>
where
    Acc: NumExpr,
    Acc::Ret: NumberVal,
    B<DBs, DB>: NumberVal,
{
    type Ret = Acc::Ret;
}

impl<NBs, NB, DBs, DB, Acc> NumExpr for DivExp<B<NBs, NB>, B<DBs, DB>, Acc>
where
    Acc: NumExpr,
    AddExp<Acc::Ret, _1>: NumExpr,
    B<NBs, NB>: NumExpr,
    B<DBs, DB>: NumExpr,
    NumRet<B<NBs, NB>>: NumberVal,
    NumRet<B<DBs, DB>>: NumberVal,
    LT<B<NBs, NB>, B<DBs, DB>>: BoolExpr,
    SubExp<B<NBs, NB>, B<DBs, DB>>: NumExpr,
    IF<
        LT<B<NBs, NB>, B<DBs, DB>>,
        Acc,
        DivExp<NumRet<SubExp<B<NBs, NB>, B<DBs, DB>>>, B<DBs, DB>, AddExp<Acc::Ret, _1>>,
    >: NumExpr,
{
    type Ret = NumRet<
        IF<
            LT<B<NBs, NB>, B<DBs, DB>>,
            Acc,
            DivExp<NumRet<SubExp<B<NBs, NB>, B<DBs, DB>>>, B<DBs, DB>, AddExp<Acc::Ret, _1>>,
        >,
    >;
}
pub struct _BitwiseDiv<DlteN, Acc, N, D, M = _Recurse> {
    _lte: PhantomData<DlteN>,
    _cache: PhantomData<Acc>,
    _lhs: PhantomData<N>,
    _rhs: PhantomData<D>,
    _m: PhantomData<M>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U4, U5, U6, U7, U8},
        test_res::*,
    };
    #[test]
    fn eval_div() {
        const _0_DIV_1: () = _b0::<DivExp<U0, U1>>();
        const _2_DIV_1: () = _b2::<DivExp<U2, U1>>();
        const _1_DIV_2: () = _b0::<DivExp<U1, U2>>();
        const _3_DIV_1: () = _b3::<DivExp<U3, U1>>();
        const _1_DIV_3: () = _b0::<DivExp<U1, U3>>();
        const _2_DIV_2: () = _b1::<DivExp<U2, U2, _0>>();
        const _4_DIV_1: () = _b4::<DivExp<U4, U1>>();
        const _5_DIV_1: () = _b5::<DivExp<U5, U1>>();
        const _5_DIV_2: () = _b2::<DivExp<U5, U2>>();
        const _6_DIV_1: () = _b6::<DivExp<U6, U1>>();
        const _7_DIV_1: () = _b7::<DivExp<U7, U1>>();
        const _8_DIV_1: () = _b8::<DivExp<U8, U1>>();
        const _3_DIV_3: () = _b1::<DivExp<U3, U3>>();
    }
}
