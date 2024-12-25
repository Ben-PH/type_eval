use crate::{
    _inners::{_Base, _BitStrLit},
    num_vals::U1,
    op_types::{ShLExp, ShRExp, SubExp},
    prelude::B as BitString,
    val_types::{NumberVal, _0, _1},
    NumExpr, NumRet,
};

impl<L, R> NumExpr for ShLExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    ShLExp<L::Ret, R::Ret, _Base>: NumExpr,
{
    type Ret = NumRet<ShLExp<L::Ret, R::Ret, _Base>>;
}

impl<B> NumExpr for ShLExp<B, _0, _Base>
where
    B: NumberVal,
{
    type Ret = B;
}
impl<B, N> NumExpr for ShLExp<B, N, _Base>
where
    B: _BitStrLit,
    SubExp<N, _1>: NumExpr,
    ShLExp<BitString<B, _0>, NumRet<SubExp<N, _1>>>: NumExpr,
{
    type Ret = NumRet<ShLExp<BitString<B, _0>, NumRet<SubExp<N, _1>>>>;
}
impl<L, R> NumExpr for ShRExp<L, R>
where
    L: NumExpr,
    R: NumExpr,
    ShRExp<L::Ret, R::Ret, _Base>: NumExpr,
{
    type Ret = NumRet<ShRExp<L::Ret, R::Ret, _Base>>;
}

impl<B> NumExpr for ShRExp<B, _0, _Base>
where
    B: NumberVal,
{
    type Ret = B;
}
impl<Bs, B, N> NumExpr for ShRExp<BitString<Bs, B>, N, _Base>
where
    Bs: NumExpr,
    SubExp<N, _1>: NumExpr,
    ShRExp<Bs::Ret, NumRet<SubExp<N, _1>>, _Base>: NumExpr,
{
    type Ret = NumRet<ShRExp<Bs::Ret, NumRet<SubExp<N, _1>>, _Base>>;
}
impl<N> NumExpr for ShRExp<U1, N, _Base>
where
    SubExp<N, _1>: NumExpr,
{
    type Ret = _0;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        num_vals::{U0, U1, U2, U3, U9},
        test_res::*,
    };
    #[allow(non_upper_case_globals)]
    #[test]
    fn eval_msb() {
        const _0_ShL_0: () = _b0::<ShLExp<_0, U0>>();
        const _1_ShL_0: () = _b1::<ShLExp<_1, U0>>();
        const _1_ShL_1: () = _b2::<ShLExp<_1, U1>>();
        const _1_ShR_1: () = _b0::<ShRExp<_1, U1>>();
        const _1_ShR_0: () = _b1::<ShRExp<_1, U0>>();

        const _10_ShR_0: () = _b2::<ShRExp<BitString<_1, _0>, U0>>();
        const _11_ShR_0: () = _b3::<ShRExp<BitString<_1, _1>, U0>>();
        const _10_ShR_1: () = _b1::<ShRExp<BitString<_1, _0>, U1>>();
        const _11_ShR_1: () = _b1::<ShRExp<BitString<_1, _1>, U1>>();

        const _1_ShL_2: () = _b1::<ShLExp<_1, U0>>();
        const _1_ShL_3: () = _b2::<ShLExp<_1, U1>>();
        const _3_ShR_2: () = _b0::<ShRExp<U3, U2>>();
        const _9_ShR_3: () = _b1::<ShRExp<U9, U3>>();
        // const _MSB_10: () = _b1::<MSB<BitString<_1, _0>>>();
        // const _MSB_11: () = _b1::<MSB<BitString<_1, _1>>>();
        // const _MSB_100: () = _b1::<MSB<BitString<BitString<_1, _0>, _1>>>();
    }
}
